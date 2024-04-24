import os
from utils import parse_file, process_project_folder


if __name__ == '__main__':
    root_folder = 'zkevm-circuits/zkevm-circuits/'
    output_folder = 'testing/tree-sitter-result/zkevm-circuits'
    process_project_folder(root_folder, output_folder)
