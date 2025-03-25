#!/usr/bin/env bash

#----------------------------#
# Usage
#----------------------------#
if [[ "$#" -lt 1 ]]; then
    cat <<EOF
Usage: $0 <package_name>

Execute test cases defined in the package's JSON file from packages/ directory.
EOF
    exit 1
fi

#----------------------------#
# Functions
#----------------------------#
function run_test() {
    local name=$1
    local cmd=$2
    local ignore_exit_code=$3
    local pattern=$4
    shift 4
    local args=("$@")

    echo -n "==> Running test '$name'... "
    
    # Execute command and capture output
    local output
    output=$($cmd "${args[@]}" 2>&1)
    local exit_code=$?

    # Check if command executed successfully
    if [[ $exit_code -ne 0 && "$ignore_exit_code" != "true" ]]; then
        echo "FAILED"
        echo "Command failed with exit code $exit_code"
        echo "Output: $output"
        return 1
    fi

    # If pattern is specified, check for match
    if [[ -n "$pattern" ]]; then
        if echo "$output" | grep -qE "$pattern"; then
            echo "PASSED"
            return 0
        else
            echo "FAILED"
            echo "Output '$output' does not match pattern '$pattern'"
            return 1
        fi
    else
        echo "PASSED"
        return 0
    fi
}

#----------------------------#
# Main
#----------------------------#
package_name=$1
json_file="packages/${package_name}.json"

# Check if file exists
if [[ ! -f "$json_file" ]]; then
    echo "Error: Package definition file $json_file not found"
    exit 1
fi

# Check if jq command is available
if ! command -v jq &> /dev/null; then
    echo "Error: jq command is required"
    exit 1
fi

# Get number of test cases
test_count=$(jq '.tests | length' "$json_file")
if [[ $test_count -eq 0 ]]; then
    echo "Warning: No test cases defined in $json_file"
    exit 0
fi

# Execute all test cases
failed=0
for ((i=0; i<test_count; i++)); do
    # Get test case fields and validate
    name=$(jq -r ".tests[$i].name // \"test #$i\"" "$json_file")
    cmd=$(jq -r ".tests[$i].command" "$json_file")
    ignore_exit_code=$(jq -r ".tests[$i].ignore_exit_code // false" "$json_file")
    pattern=$(jq -r ".tests[$i].pattern // empty" "$json_file")

    # Check required fields
    if [[ -z "$cmd" ]]; then
        echo "Error: Test case #$i missing 'command' field"
        exit 1
    fi

    # Add cbp bin prefix to command if it's not a system command
    case "$cmd" in
        ls|cat|grep|find|which|test|mkdir|rm|cp|mv)
            # System commands, use as-is
            ;;
        *)
            # Package commands, add cbp prefix
            cmd="$(cbp prefix bin)/$cmd"
            ;;
    esac
    
    # Get argument array
    args=()
    if jq -e ".tests[$i].args" "$json_file" > /dev/null; then
        if jq -e 'type == "array"' <<< "$(jq ".tests[$i].args" "$json_file")" > /dev/null; then
            while IFS= read -r arg; do
                # Evaluate the argument if it contains command substitution
                if [[ $arg == *'$(cbp'* ]]; then
                    arg=$(eval echo "$arg")
                fi
                args+=("$arg")
            done < <(jq -r ".tests[$i].args[]" "$json_file")
        else
            arg=$(jq -r ".tests[$i].args" "$json_file")
            # Evaluate the argument if it contains command substitution
            if [[ $arg == *'$(cbp'* ]]; then
                arg=$(eval echo "$arg")
            fi
            args=("$arg")
        fi
    fi

    if ! run_test "$name" "$cmd" "$ignore_exit_code" "$pattern" "${args[@]}"; then
        ((failed++))
    fi
done

# Output summary
echo
if [[ $failed -eq 0 ]]; then
    echo "✅ All tests passed"
    exit 0
else
    echo "❌ $failed tests failed"
    exit 1
fi
