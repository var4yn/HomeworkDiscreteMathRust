import OkOrInvalidComponent from "./OkOrInvalidComponent";

const InputBooleanFunction = ( {inputValue, isOk, handleInput, placeholder, maxLength} ) => {
    return (
        <>
            <input placeholder={placeholder ? placeholder : "Введите булевую функцию"} value={inputValue} maxLength={maxLength ? maxLength : "32"} onInput={handleInput}></input>
            <div className="px-3 py-1"><OkOrInvalidComponent condition={isOk}/></div>
        </>
    )
}

export default InputBooleanFunction;