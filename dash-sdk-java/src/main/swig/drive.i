%rename (WhereOperator) drive_query_conditions_WhereOperator;
START_CLASS(WhereClause, drive_query_conditions_WhereClause);
    drive_query_conditions_WhereClause(char * property, drive_query_conditions_WhereOperator op, platform_value_Value * value) {
        return drive_query_conditions_WhereClause_ctor(
            memoryFactory.clone(property),
            clone(&op),
            clone(value)
        );
    }
    drive_query_conditions_WhereOperator getOperator() {
        return *$self->operator_;
    }
END_CLASS();
%ignore drive_query_conditions_WhereClause::operator_;

START_CLASS(OrderClause, drive_query_ordering_OrderClause);
    drive_query_ordering_OrderClause(char * property, bool ascending) {
        return drive_query_ordering_OrderClause_ctor(
            memoryFactory.clone(property),
            ascending
        );
    }
    drive_query_ordering_OrderClause(char * property) {
            return drive_query_ordering_OrderClause_ctor(
                memoryFactory.clone(property),
                true // ascending
            );
        }
END_CLASS();

START_CLASS(Start, platform_mobile_fetch_document_StartPoint);
    platform_mobile_fetch_document_StartPoint(Vec_u8 * bytes, bool startAt) {
        if (startAt) return platform_mobile_fetch_document_StartPoint_StartAt_ctor(clone(bytes));
        else return platform_mobile_fetch_document_StartPoint_StartAfter_ctor(clone(bytes));
    }
END_CLASS();

LIST_STRUCT_TYPEMAP(Vec_drive_query_conditions_WhereClause, drive_query_conditions_WhereClause, WhereClause, clone);
LIST_STRUCT_TYPEMAP(Vec_drive_query_ordering_OrderClause, drive_query_ordering_OrderClause, OrderClause, clone);