
// struct Vec_dpp_document_Document {
//     uintptr_t count;
//     dpp_document_Document **values;
// };

LIST_STRUCT_TYPEMAP(Vec_dpp_document_Document, dpp_document_Document, Document);
%ignore Vec_dpp_document_Document;

START_CLASS(Document, dpp_document_Document);
END_CLASS();

START_CLASS(DocumentV0, dpp_document_v0_DocumentV0);
END_CLASS();