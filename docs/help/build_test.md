Execute test cases defined in package configuration files.

Usage:
  cbp build test <PACKAGES>...     # Test specified packages

Test cases are defined in package configuration files under the "tests" section:
  {
    "tests": [
      {
        "name": "test name",       # Optional, defaults to "test #N"
        "command": "command",      # Required, command to execute
        "args": ["arg1", "arg2"],  # Optional, command arguments
        "pattern": "regex",        # Optional, pattern to match in output
        "ignore_exit_code": false  # Optional, ignore command exit code
      }
    ]
  }
