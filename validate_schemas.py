#!/usr/bin/env python3
"""
Schema Validation Script for Lang4LLM Research Deliverables

Validates all JSON/YAML files against schemas defined in
specs/001-llm-lang-research/contracts/deliverable-schemas.yaml
"""

import yaml
import json
import re
from pathlib import Path
from typing import Dict, List, Tuple, Any
from collections import defaultdict


class SchemaValidator:
    def __init__(self, schema_path: Path):
        with open(schema_path, 'r') as f:
            schema_doc = yaml.safe_load(f)
        self.schemas = schema_doc['components']['schemas']
        self.errors = []
        self.warnings = []

    def validate_error_pattern(self, file_path: Path, data: Dict) -> List[str]:
        """Validate ErrorPattern schema"""
        errors = []
        schema = self.schemas['ErrorPattern']

        # Required fields
        required = schema['required']
        for field in required:
            if field not in data:
                errors.append(f"Missing required field: {field}")

        # ID pattern
        if 'id' in data:
            pattern = r'^EP-(SYN|SEM|LOG)-\d{3}$'
            if not re.match(pattern, data['id']):
                errors.append(f"Invalid ID format: {data['id']} (expected {pattern})")

        # Category enum
        if 'category' in data:
            valid_categories = ['SYNTAX', 'SEMANTIC', 'LOGIC']
            if data['category'] not in valid_categories:
                errors.append(f"Invalid category: {data['category']} (expected one of {valid_categories})")

        # Frequency data
        if 'frequency' in data:
            freq = data['frequency']
            if 'overall' not in freq:
                errors.append("frequency.overall is required")
            elif not (0 <= freq['overall'] <= 1):
                errors.append(f"frequency.overall must be in [0,1], got {freq['overall']}")

        # Trigger conditions
        if 'trigger_conditions' in data:
            for i, tc in enumerate(data['trigger_conditions']):
                if 'feature' not in tc:
                    errors.append(f"trigger_conditions[{i}].feature is required")
                if 'severity' not in tc:
                    errors.append(f"trigger_conditions[{i}].severity is required")
                elif tc['severity'] not in ['HIGH', 'MEDIUM', 'LOW']:
                    errors.append(f"trigger_conditions[{i}].severity invalid: {tc['severity']}")

        # Examples
        if 'examples' in data:
            for i, ex in enumerate(data['examples']):
                required_ex = ['language', 'incorrect', 'correct', 'explanation']
                for field in required_ex:
                    if field not in ex:
                        errors.append(f"examples[{i}].{field} is required")
                if 'language' in ex and ex['language'] not in ['python', 'rust', 'typescript', 'go']:
                    errors.append(f"examples[{i}].language invalid: {ex['language']}")

        # Sources
        if 'sources' in data:
            for i, src in enumerate(data['sources']):
                if 'paper_id' not in src:
                    errors.append(f"sources[{i}].paper_id is required")

        return errors

    def validate_design_rule(self, file_path: Path, data: Dict) -> List[str]:
        """Validate DesignRule schema"""
        errors = []
        schema = self.schemas['DesignRule']

        # Required fields
        required = schema['required']
        for field in required:
            if field not in data:
                errors.append(f"Missing required field: {field}")

        # ID pattern
        if 'id' in data:
            pattern = r'^DR-(SYN|GRM|SEM|TOK|ERR)-\d{3}$'
            if not re.match(pattern, data['id']):
                errors.append(f"Invalid ID format: {data['id']} (expected {pattern})")

        # Category enum
        if 'category' in data:
            valid_categories = ['SYNTAX', 'GRAMMAR', 'SEMANTIC', 'TOKEN', 'ERROR_PREVENTION']
            if data['category'] not in valid_categories:
                errors.append(f"Invalid category: {data['category']} (expected one of {valid_categories})")

        # Priority
        if 'priority' in data:
            if not isinstance(data['priority'], int) or data['priority'] < 1:
                errors.append(f"priority must be integer >= 1, got {data['priority']}")

        # Impact score
        if 'impact_score' in data:
            if not (0 <= data['impact_score'] <= 1):
                errors.append(f"impact_score must be in [0,1], got {data['impact_score']}")

        # Research backing
        if 'research_backing' in data:
            for i, rb in enumerate(data['research_backing']):
                required_rb = ['source', 'finding', 'confidence']
                for field in required_rb:
                    if field not in rb:
                        errors.append(f"research_backing[{i}].{field} is required")
                if 'confidence' in rb and rb['confidence'] not in ['HIGH', 'MEDIUM', 'LOW']:
                    errors.append(f"research_backing[{i}].confidence invalid: {rb['confidence']}")

        # Related patterns
        if 'related_patterns' in data:
            for i, rp in enumerate(data['related_patterns']):
                required_rp = ['pattern_id', 'mitigation']
                for field in required_rp:
                    if field not in rp:
                        errors.append(f"related_patterns[{i}].{field} is required")
                if 'mitigation' in rp and rp['mitigation'] not in ['PREVENTS', 'REDUCES', 'DETECTS']:
                    errors.append(f"related_patterns[{i}].mitigation invalid: {rp['mitigation']}")

        return errors

    def validate_metric_data(self, file_path: Path, data: Dict) -> List[str]:
        """Validate Metric data files (language scores)"""
        errors = []

        # Check for metadata
        if 'metadata' not in data:
            errors.append("Missing metadata section")

        # Check for metric sections (M-001 through M-004)
        expected_metrics = ['M-001_token_predictability', 'M-002_parse_ambiguity',
                           'M-003_ast_complexity', 'M-004_error_recovery']

        for metric in expected_metrics:
            if metric not in data:
                errors.append(f"Missing metric section: {metric}")
            else:
                metric_data = data[metric]
                if 'metric_name' not in metric_data:
                    errors.append(f"{metric}.metric_name is required")
                if 'language_score' not in metric_data:
                    errors.append(f"{metric}.language_score is required")

        return errors

    def validate_file(self, file_path: Path, file_type: str) -> Tuple[bool, List[str]]:
        """Validate a single file against its schema"""
        try:
            # Load file
            with open(file_path, 'r') as f:
                if file_path.suffix == '.json':
                    data = json.load(f)
                elif file_path.suffix in ['.yaml', '.yml']:
                    data = yaml.safe_load(f)
                elif file_path.suffix == '.md':
                    # Parse YAML frontmatter or structured markdown
                    content = f.read()
                    # Look for YAML frontmatter
                    if content.startswith('---'):
                        parts = content.split('---', 2)
                        if len(parts) >= 2:
                            data = yaml.safe_load(parts[1])
                        else:
                            return True, []  # No structured data
                    else:
                        return True, []  # No structured data in markdown
                else:
                    return True, []  # Unknown format

            # Validate based on type
            if file_type == 'ErrorPattern':
                errors = self.validate_error_pattern(file_path, data)
            elif file_type == 'DesignRule':
                errors = self.validate_design_rule(file_path, data)
            elif file_type == 'MetricData':
                errors = self.validate_metric_data(file_path, data)
            else:
                return True, []  # No validator for this type yet

            return len(errors) == 0, errors

        except Exception as e:
            return False, [f"Parse error: {str(e)}"]


def main():
    """Run validation on all research deliverable files"""
    base_dir = Path(__file__).parent
    schema_path = base_dir / "specs/001-llm-lang-research/contracts/deliverable-schemas.yaml"

    if not schema_path.exists():
        print(f"❌ Schema file not found: {schema_path}")
        return 1

    validator = SchemaValidator(schema_path)

    # Define file sets to validate
    file_sets = {
        'ErrorPattern': list((base_dir / "research/D1-error-taxonomy/patterns").rglob("*.yaml")),
        'DesignRule': list((base_dir / "research/D2-design-guide/rules").rglob("*.yaml")),
        'MetricData': list((base_dir / "research/D3-metrics/data").glob("*.json")),
    }

    # Track results
    results = defaultdict(lambda: {'passed': 0, 'failed': 0, 'errors': []})

    print("=" * 80)
    print("Schema Validation Report")
    print("=" * 80)
    print()

    # Validate each file set
    for file_type, files in file_sets.items():
        print(f"\n{'=' * 80}")
        print(f"{file_type} Files ({len(files)} files)")
        print(f"{'=' * 80}\n")

        for file_path in sorted(files):
            relative_path = file_path.relative_to(base_dir)
            passed, errors = validator.validate_file(file_path, file_type)

            if passed:
                results[file_type]['passed'] += 1
                print(f"✅ {relative_path}")
            else:
                results[file_type]['failed'] += 1
                print(f"❌ {relative_path}")
                for error in errors:
                    print(f"   - {error}")
                results[file_type]['errors'].append((str(relative_path), errors))

    # Summary
    print("\n" + "=" * 80)
    print("Summary")
    print("=" * 80 + "\n")

    total_passed = 0
    total_failed = 0

    for file_type, stats in results.items():
        total_passed += stats['passed']
        total_failed += stats['failed']
        print(f"{file_type}:")
        print(f"  ✅ Passed: {stats['passed']}")
        print(f"  ❌ Failed: {stats['failed']}")
        print()

    print(f"Overall: {total_passed} passed, {total_failed} failed")

    if total_failed > 0:
        print(f"\n⚠️  {total_failed} files failed validation")
        return 1
    else:
        print("\n✅ All files passed validation!")
        return 0


if __name__ == '__main__':
    exit(main())
