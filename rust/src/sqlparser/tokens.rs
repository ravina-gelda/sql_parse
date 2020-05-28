pub enum Token{
    word,
    space,
    semicolon,

    
}
pub enum Step {
    stepType,
    selectField,
    selectFrom,
    selectComma,
    validateSelectFromOrComma,
    selectFromTable,
    insertTable,
    validateInto,
    validateFrom,
    insertFieldsOpeningParens,
    insertFieldsCommaOrClosingParens,
    insertFieldsValuesOpeningParens,
    insertValuesRWord,
    insertValues,
    insertValuesCommaOrClosingParens,
    insertValuesCommaBeforeOpeningParens,
    updateTable,
    updateSet,
    updateField,
    updateEquals,
    updateValue,
    updateComma,
    deleteFromTable,
    stepWhere,
    whereField,
    whereOperator,
    whereValue,
    whereAnd

}
