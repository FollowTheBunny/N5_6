#!/bin/bash

# Directory containing the test cases and expected results
TEST_DIR="../examples"

# Loop through test cases
for i in {0..24}
do
    # Construct the file names
    TEST_FILE="${TEST_DIR}/example${i}.txt"
    EXPECTED_FILE="${TEST_DIR}/example${i}.expected.txt"
    OUTPUT_FILE="/out.txt"

    # Check if the test file exists
    if [ -f "$TEST_FILE" ]; then
        echo "Testing example${i}.txt"

        # Run the Kotlin program with the test file as input and redirect the output to a file
        ./gradlew run --args="../\"$TEST_FILE\" \"$OUTPUT_FILE\""

        # Compare the output with the expected result
        if cmp -s "app/$OUTPUT_FILE" "$EXPECTED_FILE"; then
            echo "Test example${i}.txt passed."
        else
            echo "Test example${i}.txt failed."
            echo "Difference:"
            diff "$OUTPUT_FILE" "$EXPECTED_FILE"
        fi
    else
        echo "Test file example${i}.txt not found."
    fi
done

