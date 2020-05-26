pub enum Tokens{
    
}
pub enum Step {
    stepType,
    selectField,
    selectFrom,
    selectComma,
    selectFromTable,
    insertTable,
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
    where,
    whereField,
    whereOperator,
    whereValue,
    whereAnd

}