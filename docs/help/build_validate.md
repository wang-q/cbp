Validate package configuration files against JSON schema.

Checks that package JSON files conform to the expected schema,
ensuring all required fields are present and correctly typed.

Examples:
1. Validate packages:
   cbp build validate zlib

2. Validate multiple packages:
   cbp build validate zlib bzip2

3. Use custom schema:
   cbp build validate zlib --schema custom-schema.json

4. Specify base directory:
   cbp build validate zlib --base /path/to/project