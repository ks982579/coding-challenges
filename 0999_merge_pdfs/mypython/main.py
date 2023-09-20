"""
Super simple example: https://pypdf2.readthedocs.io/en/latest/user/merging-pdfs.html
"""

# pdf_merging.py

import os
from pathlib import Path
from PyPDF2 import PdfWriter, PdfReader

def merge_pdfs(pdf_path, output):
    pdf_files = [Path(os.path.join(pdf_path, file)).resolve() for file in os.listdir(pdf_path) if file.endswith(".pdf")]
    pdf_files.sort()
    print(pdf_files)
    pdf_writer = PdfWriter()

    for path in pdf_files:
        pdf_writer.append(path)

    # Write out the merged PDF
    with open(output, 'wb') as out:
        pdf_writer.write(out)

if __name__ == '__main__':
    pdf_path = "../cache"
    merge_pdfs(pdf_path, output='merged.pdf')
