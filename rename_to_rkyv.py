
import os
import re
import shutil

def modify_rust_file(filepath):
    """
    Modifies a single Rust file to replace serde imports, derive attributes,
    remove entire ros2_client:: implementations blocks, remove
    #[serde(rename = "...")] attributes on fields, remove any line
    containing 'serde_as', and replace all occurrences of 'type_' with 'r#type'.

    Args:
        filepath (str): The path to the Rust file to modify.
    """
    print(f"Processing file: {filepath}")

    # Define the patterns and replacements
    serde_import_pattern = r'use serde::{Deserialize, Serialize};'
    rkyv_import_replacement = 'use rkyv::{Archive, Deserialize, Serialize};'

    # Pattern to find #[derive(...)] and replace Serialize, Deserialize
    # This regex captures:
    # Group 1: The start of the derive attribute up to the opening parenthesis (e.g., #[derive() ).
    # Group 2: Anything inside the parentheses *before* 'Serialize, Deserialize'.
    # Group 3: Anything inside the parentheses *after* 'Serialize, Deserialize'.
    # Updated pattern to correctly capture content within parentheses and handle whitespace.
    derive_pattern = r'(#[\[]derive\()([^)]*?)\s*Serialize\s*,\s*Deserialize\s*([^)]*?)\)]'
    derive_replacement = r'\1\2Serialize, Deserialize, Archive\3)]'

    # Pattern to find and remove entire 'impl ros2_client::... { ... }' blocks
    # This pattern matches from 'impl ros2_client::' up to the matching closing brace '}'
    # It uses re.DOTALL to match across multiple lines.
    # The regex is more robust to content within the braces, including simple nested braces.
    ros2_client_impl_block_pattern = r'impl\s+ros2_client::.*?\{\s*(?:[^{}]|{[^{}]*})*\s*\}\s*'

    # Pattern to find and remove #[serde(rename = "...")] attributes on fields
    # This pattern looks for #[serde(rename = "...")] followed by potential whitespace
    # before the field definition starts.
    serde_rename_pattern = r'#[\[]serde\(\s*rename\s*=\s*".*?"\s*\)\]\s*'

    # Pattern to find and remove any line containing 'serde_as'
    # This matches any characters at the start of a line, followed by 'serde_as',
    # followed by any characters until the end of the line.
    # We use re.M flag for multiline mode so '^' and '$' match start/end of each line.
    line_with_serde_as_pattern = r'^.*serde_as.*$\n?'
    # Replacement is an empty string to delete the line (including the newline character if present).

    # Pattern to find and replace all occurrences of 'type_' with 'r#type'
    # Use word boundaries (\b) to ensure we only match the whole word 'type_'
    type_underscore_pattern = r'\btype_\b'
    type_raw_replacement = r'r#type'


    try:
        # Read the original file content
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()

        # Perform the replacements

        # 1. Remove any line containing 'serde_as'
        # This is done early to clean up before other replacements.
        # Use re.M flag for multiline matching.
        modified_content = re.sub(line_with_serde_as_pattern, '', content, flags=re.M)

        # 2. Remove #[serde(rename = "...")] attributes
        # This ensures they are removed before other patterns might interfere.
        modified_content = re.sub(serde_rename_pattern, '', modified_content)

        # 3. Replace the import line
        modified_content = re.sub(serde_import_pattern, rkyv_import_replacement, modified_content)

        # 4. Replace the derive attributes
        # Use re.DOTALL flag to allow '.' to match newline characters, in case derive spans multiple lines
        modified_content = re.sub(derive_pattern, derive_replacement, modified_content, flags=re.DOTALL)

        # 5. Remove the entire ros2_client:: implementation blocks
        # Replace the matched pattern with an empty string
        # Use re.DOTALL flag to allow '.' to match newline characters
        modified_content = re.sub(ros2_client_impl_block_pattern, '', modified_content, flags=re.DOTALL)

        # 6. Rename all occurrences of 'type_' to 'r#type'
        # This replaces the previous specific 'pub type_' rule.
        modified_content = re.sub(type_underscore_pattern, type_raw_replacement, modified_content)


        # Write the modified content back to the file
        # Using a temporary file and renaming is safer than writing directly
        temp_filepath = filepath + '.tmp'
        with open(temp_filepath, 'w', encoding='utf-8') as f:
            f.write(modified_content)

        # Replace the original file with the modified one
        shutil.move(temp_filepath, filepath)

        print(f"Successfully modified: {filepath}")

    except FileNotFoundError:
        print(f"Error: File not found at {filepath}")
    except Exception as e:
        print(f"Error processing file {filepath}: {e}")

def find_and_modify_rust_files(root_dir):
    """
    Walks through directories starting from root_dir, finds all .rs files,
    and calls modify_rust_file on each.

    Args:
        root_dir (str): The starting directory to search from.
    """
    print(f"Starting search in: {root_dir}")
    for dirpath, dirnames, filenames in os.walk(root_dir):
        for filename in filenames:
            if filename.endswith('.rs'):
                filepath = os.path.join(dirpath, filename)
                modify_rust_file(filepath)
    print("Script finished.")

# --- Script Entry Point ---
if __name__ == "__main__":
    # Get the current directory where the script is run from
    starting_directory = "." # You can change this to a specific path if needed

    find_and_modify_rust_files(starting_directory)
