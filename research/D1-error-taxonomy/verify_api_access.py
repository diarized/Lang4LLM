#!/usr/bin/env python3
"""
Verify LLM API Access for Error Taxonomy Research

Tests connectivity to GPT-4, Claude, and DeepSeek APIs.
Ensures API keys are configured and models are accessible.

Usage:
    python verify_api_access.py

Requirements:
    - openai>=1.0.0
    - anthropic>=0.18.0
    - requests>=2.31.0
    - python-dotenv>=1.0.0
"""

import os
import sys
import json
from pathlib import Path
from typing import Dict, Optional

try:
    from dotenv import load_dotenv
except ImportError:
    print("⚠ Warning: python-dotenv not installed. Install with: pip install python-dotenv")
    load_dotenv = lambda: None


def load_config() -> Dict:
    """Load API configuration from template."""
    config_path = Path(__file__).parent / "api-config.template.json"
    if not config_path.exists():
        print(f"✗ Config file not found: {config_path}")
        sys.exit(1)

    with open(config_path) as f:
        return json.load(f)


def check_api_key(env_var: str) -> bool:
    """Check if API key environment variable is set."""
    key = os.getenv(env_var)
    if not key:
        print(f"  ✗ {env_var} not set")
        return False

    # Basic validation: check key format
    if len(key) < 20:
        print(f"  ⚠ {env_var} seems too short (might be invalid)")
        return False

    print(f"  ✓ {env_var} found ({len(key)} chars)")
    return True


def test_openai_api() -> bool:
    """Test OpenAI API access."""
    try:
        import openai
        client = openai.OpenAI(api_key=os.getenv("OPENAI_API_KEY"))

        # Simple test: list models
        models = client.models.list()
        print(f"  ✓ OpenAI API accessible ({len(list(models.data))} models available)")
        return True
    except ImportError:
        print("  ⚠ openai package not installed. Install with: pip install openai")
        return False
    except Exception as e:
        print(f"  ✗ OpenAI API error: {e}")
        return False


def test_anthropic_api() -> bool:
    """Test Anthropic API access."""
    try:
        import anthropic
        client = anthropic.Anthropic(api_key=os.getenv("ANTHROPIC_API_KEY"))

        # Simple test: send minimal request
        response = client.messages.create(
            model="claude-3-5-sonnet-20241022",
            max_tokens=10,
            messages=[{"role": "user", "content": "Hi"}]
        )
        print(f"  ✓ Anthropic API accessible (response: {len(response.content[0].text)} chars)")
        return True
    except ImportError:
        print("  ⚠ anthropic package not installed. Install with: pip install anthropic")
        return False
    except Exception as e:
        print(f"  ✗ Anthropic API error: {e}")
        return False


def test_deepseek_api() -> bool:
    """Test DeepSeek API access."""
    try:
        import requests

        api_key = os.getenv("DEEPSEEK_API_KEY")
        if not api_key:
            print("  ⚠ DEEPSEEK_API_KEY not set, skipping test")
            return False

        # DeepSeek uses OpenAI-compatible API
        headers = {
            "Authorization": f"Bearer {api_key}",
            "Content-Type": "application/json"
        }

        # Note: Actual endpoint may vary, this is a placeholder
        # Update with real DeepSeek API endpoint
        print("  ⚠ DeepSeek API test not implemented (update with real endpoint)")
        return False

    except ImportError:
        print("  ⚠ requests package not installed. Install with: pip install requests")
        return False
    except Exception as e:
        print(f"  ✗ DeepSeek API error: {e}")
        return False


def main():
    """Main verification routine."""
    print("=" * 60)
    print("LLM API Access Verification")
    print("=" * 60)

    # Load environment variables
    load_dotenv()

    # Load configuration
    config = load_config()
    print(f"\n✓ Configuration loaded: {len(config['models'])} models defined\n")

    results = {}

    # Check each model
    for model_name, model_config in config["models"].items():
        print(f"\n{model_name.upper()}:")
        print("-" * 40)

        # Check API key
        env_var = model_config["api_key_env"]
        key_ok = check_api_key(env_var)

        # Test API access
        api_ok = False
        if key_ok:
            if model_name == "gpt4":
                api_ok = test_openai_api()
            elif model_name == "claude":
                api_ok = test_anthropic_api()
            elif model_name == "deepseek":
                api_ok = test_deepseek_api()

        results[model_name] = key_ok and api_ok

    # Summary
    print("\n" + "=" * 60)
    print("SUMMARY")
    print("=" * 60)

    all_ok = all(results.values())
    for model_name, status in results.items():
        symbol = "✓" if status else "✗"
        print(f"{symbol} {model_name.upper()}: {'READY' if status else 'NOT READY'}")

    print("\n" + "=" * 60)

    if all_ok:
        print("✓ All APIs are configured and accessible")
        print("\nNext steps:")
        print("  1. Run T022-T024: Download benchmark datasets")
        print("  2. Run T025-T027: Execute LLM evaluations")
        return 0
    else:
        print("✗ Some APIs are not ready")
        print("\nTroubleshooting:")
        print("  1. Ensure API keys are set in .env file:")
        print("     OPENAI_API_KEY=sk-...")
        print("     ANTHROPIC_API_KEY=sk-ant-...")
        print("     DEEPSEEK_API_KEY=...")
        print("  2. Install required packages:")
        print("     pip install openai anthropic requests python-dotenv")
        print("  3. Check API key validity on provider dashboards")
        return 1


if __name__ == "__main__":
    sys.exit(main())
