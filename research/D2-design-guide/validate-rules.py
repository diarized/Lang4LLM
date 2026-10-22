#!/usr/bin/env python3
"""
Validate D2 design rule files against schema (T051 + T052)
"""

import yaml
import glob
import sys
from pathlib import Path

def validate_rule(rule_path):
    """Validate a single rule file against requirements"""
    with open(rule_path, 'r') as f:
        rule = yaml.safe_load(f)

    errors = []

    # Required fields
    required = ['id', 'category', 'name', 'description', 'priority', 'impact_score',
                'rationale', 'research_backing', 'validation_method', 'related_patterns',
                'implementation_notes']

    for field in required:
        if field not in rule:
            errors.append(f"Missing required field: {field}")

    if errors:
        return False, errors

    # Validate ID pattern
    id_pattern = rule['id']
    if not id_pattern.startswith('DR-'):
        errors.append(f"ID must start with 'DR-': {id_pattern}")

    # Validate category
    valid_categories = ['SYNTAX', 'GRAMMAR', 'SEMANTIC', 'TOKEN', 'ERROR_PREVENTION']
    if rule['category'] not in valid_categories:
        errors.append(f"Invalid category: {rule['category']}")

    # Validate priority range
    if not (1 <= rule['priority'] <= 20):
        errors.append(f"Priority must be 1-20: {rule['priority']}")

    # Validate impact_score range
    if not (0 <= rule['impact_score'] <= 1):
        errors.append(f"Impact score must be 0-1: {rule['impact_score']}")

    # Validate research_backing exists and has entries
    if not rule['research_backing']:
        errors.append("research_backing must have at least one entry")

    # Validate validation_method has required fields
    if 'approach' not in rule['validation_method']:
        errors.append("validation_method must have 'approach' field")

    # Validate related_patterns has entries
    if not rule['related_patterns']:
        errors.append("related_patterns must have at least one entry")

    return len(errors) == 0, errors

def main():
    """Validate all rule files"""
    rule_dir = Path(__file__).parent / 'rules'

    all_valid = True
    rule_count = 0
    rules_by_category = {}

    print("=" * 70)
    print("D2 Design Rule Validation (T051 + T052)")
    print("=" * 70)
    print()

    # Find all YAML files in rules subdirectories
    for category_dir in ['syntax', 'grammar', 'semantic', 'token', 'error-prevention']:
        pattern = rule_dir / category_dir / '*.yaml'
        category_rules = list(glob.glob(str(pattern)))

        if not category_rules:
            print(f"⚠️  No rules found in {category_dir}/")
            continue

        rules_by_category[category_dir] = []

        for rule_path in sorted(category_rules):
            rule_count += 1
            rule_file = Path(rule_path).name

            valid, errors = validate_rule(rule_path)

            with open(rule_path, 'r') as f:
                rule = yaml.safe_load(f)

            if valid:
                print(f"✓ {rule['id']:12} {rule['name'][:45]:45} (P{rule['priority']:02}, I={rule['impact_score']:.2f})")
                rules_by_category[category_dir].append(rule)
            else:
                print(f"✗ {rule_file}")
                for error in errors:
                    print(f"    - {error}")
                all_valid = False

    print()
    print("=" * 70)
    print(f"Summary")
    print("=" * 70)
    print(f"Total Rules: {rule_count}")

    for category, rules in rules_by_category.items():
        print(f"  {category.upper():20} {len(rules)} rules")

    print()

    if all_valid and rule_count >= 20:
        print("✅ VALIDATION PASSED")
        print(f"   - All {rule_count} rules are valid")
        print("   - All required fields present")
        print("   - All rules have research backing")
        print("   - All rules have validation methods")
        print("   - All rules link to error patterns")
        return 0
    elif all_valid:
        print(f"⚠️  VALIDATION WARNING: Only {rule_count} rules (expected 20+)")
        return 1
    else:
        print("❌ VALIDATION FAILED")
        print("   - Some rules have errors (see above)")
        return 1

if __name__ == '__main__':
    sys.exit(main())
