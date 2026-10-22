#!/usr/bin/env python3
"""
Deliverable Validation Script

Validates research deliverable files against schemas defined in
specs/001-llm-lang-research/contracts/deliverable-schemas.yaml

Usage:
    python validate_deliverable.py <file_path> <schema_name>
    python validate_deliverable.py research/D1-error-taxonomy/patterns/syntax/EP-SYN-001.json ErrorPattern
    python validate_deliverable.py --all  # Validate all discovered deliverables
"""

import sys
import json
import yaml
import argparse
from pathlib import Path
from typing import Dict, List, Tuple

try:
    import jsonschema
except ImportError:
    print("ERROR: jsonschema not installed. Install with: pip install jsonschema")
    sys.exit(1)


class DeliverableValidator:
    """Validates deliverable files against YAML schema definitions"""

    def __init__(self, schema_file: Path):
        """Load schemas from YAML file"""
        self.schema_file = schema_file
        with open(schema_file) as f:
            schema_doc = yaml.safe_load(f)

        # Extract component schemas
        self.schemas = schema_doc.get('components', {}).get('schemas', {})
        if not self.schemas:
            raise ValueError(f"No schemas found in {schema_file}")

        print(f"✓ Loaded {len(self.schemas)} schemas from {schema_file}")

    def validate_file(self, file_path: Path, schema_name: str) -> Tuple[bool, List[str]]:
        """
        Validate a single file against a schema

        Returns:
            (is_valid, error_messages)
        """
        if schema_name not in self.schemas:
            return False, [f"Schema '{schema_name}' not found. Available: {list(self.schemas.keys())}"]

        schema = self.schemas[schema_name]

        try:
            # Load the file
            with open(file_path) as f:
                if file_path.suffix == '.json':
                    data = json.load(f)
                elif file_path.suffix in ['.yaml', '.yml']:
                    data = yaml.safe_load(f)
                else:
                    return False, [f"Unsupported file format: {file_path.suffix}"]

            # Validate against schema
            jsonschema.validate(data, schema)
            return True, []

        except jsonschema.ValidationError as e:
            return False, [f"Validation error: {e.message}", f"  Path: {' -> '.join(str(p) for p in e.path)}"]
        except json.JSONDecodeError as e:
            return False, [f"JSON parse error: {e}"]
        except yaml.YAMLError as e:
            return False, [f"YAML parse error: {e}"]
        except Exception as e:
            return False, [f"Unexpected error: {e}"]

    def discover_deliverables(self, research_dir: Path) -> Dict[str, List[Path]]:
        """
        Auto-discover deliverable files by directory pattern

        Returns:
            {schema_name: [file_paths]}
        """
        mapping = {
            'ErrorPattern': list(research_dir.glob('D1-error-taxonomy/patterns/**/*.json')),
            'DesignRule': list(research_dir.glob('D2-design-guide/rules/**/*.json')),
            'Metric': list(research_dir.glob('D3-metrics/definitions/*.json')),
            'BenchmarkSpec': list(research_dir.glob('D4-benchmark/specs/**/*.yaml')),
            'FeatureValidation': list(research_dir.glob('D5-validation/features/**/*.json')),
        }
        return {k: v for k, v in mapping.items() if v}

    def validate_all(self, research_dir: Path) -> Tuple[int, int]:
        """
        Validate all discovered deliverables

        Returns:
            (passed_count, failed_count)
        """
        discovered = self.discover_deliverables(research_dir)

        if not discovered:
            print(f"No deliverable files found in {research_dir}")
            return 0, 0

        passed = 0
        failed = 0

        for schema_name, files in discovered.items():
            print(f"\n{'='*60}")
            print(f"Validating {schema_name} ({len(files)} files)")
            print(f"{'='*60}")

            for file_path in files:
                is_valid, errors = self.validate_file(file_path, schema_name)

                if is_valid:
                    print(f"✓ {file_path.name}")
                    passed += 1
                else:
                    print(f"✗ {file_path.name}")
                    for err in errors:
                        print(f"  {err}")
                    failed += 1

        return passed, failed


def main():
    parser = argparse.ArgumentParser(
        description='Validate research deliverables against schemas',
        formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument(
        'file_path',
        nargs='?',
        help='Path to file to validate'
    )
    parser.add_argument(
        'schema_name',
        nargs='?',
        help='Schema name (ErrorPattern, DesignRule, Metric, BenchmarkSpec, FeatureValidation)'
    )
    parser.add_argument(
        '--all',
        action='store_true',
        help='Validate all discovered deliverables'
    )
    parser.add_argument(
        '--schema-file',
        default='specs/001-llm-lang-research/contracts/deliverable-schemas.yaml',
        help='Path to schema YAML file'
    )
    parser.add_argument(
        '--research-dir',
        default='research',
        help='Path to research directory'
    )

    args = parser.parse_args()

    # Determine repository root (look for .git or .beads)
    repo_root = Path.cwd()
    while repo_root != repo_root.parent:
        if (repo_root / '.git').exists() or (repo_root / '.beads').exists():
            break
        repo_root = repo_root.parent

    schema_file = repo_root / args.schema_file
    research_dir = repo_root / args.research_dir

    if not schema_file.exists():
        print(f"ERROR: Schema file not found: {schema_file}")
        sys.exit(1)

    # Initialize validator
    try:
        validator = DeliverableValidator(schema_file)
    except Exception as e:
        print(f"ERROR: Failed to load schemas: {e}")
        sys.exit(1)

    # Validate
    if args.all:
        print(f"\nScanning {research_dir} for deliverables...\n")
        passed, failed = validator.validate_all(research_dir)

        print(f"\n{'='*60}")
        print(f"SUMMARY: {passed} passed, {failed} failed")
        print(f"{'='*60}")

        sys.exit(0 if failed == 0 else 1)

    elif args.file_path and args.schema_name:
        file_path = Path(args.file_path)
        if not file_path.exists():
            print(f"ERROR: File not found: {file_path}")
            sys.exit(1)

        is_valid, errors = validator.validate_file(file_path, args.schema_name)

        if is_valid:
            print(f"✓ {file_path} is valid against {args.schema_name} schema")
            sys.exit(0)
        else:
            print(f"✗ {file_path} is invalid:")
            for err in errors:
                print(f"  {err}")
            sys.exit(1)

    else:
        parser.print_help()
        print("\nExamples:")
        print("  python validate_deliverable.py research/D1-error-taxonomy/patterns/syntax/EP-SYN-001.json ErrorPattern")
        print("  python validate_deliverable.py --all")
        sys.exit(1)


if __name__ == '__main__':
    main()
