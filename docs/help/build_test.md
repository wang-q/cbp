Execute test cases defined in package configuration files.

Test cases are defined in package configuration files under the `"tests"` section:
```
{
  "tests": [
    {
      "name": "test name",
      "command": "command",
      "args": ["arg1", "arg2"],
      "pattern": "regex",
      "ignore_exit_code": false
    }
  ]
}
```

Test case fields:
* `name` — Optional, defaults to "test #N"
* `command` — Required, command to execute
* `args` — Optional, command arguments
* `pattern` — Optional, regex pattern to match in output
* `ignore_exit_code` — Optional, ignore command exit code

Examples:
1. Test specified packages:
   `cbp build test zlib bzip2`