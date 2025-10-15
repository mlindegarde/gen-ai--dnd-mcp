#!/usr/bin/env python3
import sys
try:
    from PyPDF2 import PdfReader, PdfWriter
    
    # Read the PDF
    reader = PdfReader("docs/5E_CharacterSheet_Fillable.pdf")
    
    # Check if it has form fields
    if reader.is_encrypted:
        print("PDF is encrypted")
        sys.exit(1)
    
    # Get form fields
    if "/AcroForm" in reader.trailer["/Root"]:
        print("PDF has AcroForm")
        form = reader.trailer["/Root"]["/AcroForm"]
        if "/Fields" in form:
            fields = form["/Fields"]
            print(f"Found {len(fields)} form fields")
            
            # Try to fill a simple field
            writer = PdfWriter()
            writer.clone_reader_document_root(reader)
            
            # Fill CharacterName field
            writer.update_page_form_field_values(
                writer.pages[0], {"CharacterName": "Test Character"}
            )
            
            # Save test
            with open("python_test.pdf", "wb") as output_file:
                writer.write(output_file)
            print("Created python_test.pdf with filled field")
        else:
            print("No /Fields in AcroForm")
    else:
        print("PDF has no AcroForm")
        
except ImportError:
    print("PyPDF2 not available")
except Exception as e:
    print(f"Error: {e}")
