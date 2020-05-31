pub enum Token{
    word,
    space,
    semicolon,

    
}
pub enum Step {
    stepType,
    selectField,
    validateSelectFromOrComma,
    selectFromTable,
    insertTable,
    insertFieldsOpeningParens,
    insertFieldsCommaOrClosingParens,
    insertFields,
    insertValuesRWord,
    insertValues,
    insertValuesOpeningParens,
    insertValuesCommaOrClosingParens,
    insertValuesCommaBeforeOpeningParens,
    updateTable,
    updateSet,
    updateField,
    updateEquals,
    updateValue,
    deleteFromTable,
    stepWhere,
    whereField,
    whereOperator,
    whereValue,
    whereAnd,
    validateUpdateCommaOrWhere,

}
