import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import { MxArgRender } from "../utils/renders";
import { useInputHandler } from "../utils/handlers";
import OutputComponent from "../components/OutputComponent";
import OkOrInvalidComponent from "../components/OkOrInvalidComponent";
import InputBooleanFunction from "../components/InputBooleanFunction";



const Task2 = () => {

    const {
        inputValue, isOk, mxArg, handleInput
    } = useInputHandler( {mn: 1} );

    const [numArg, setNumArg] = useState("");

    async function handleArgsInput(event) {
        const value = event.target.value;
        const ch = value[value.length - 1];
        if ( (ch >= "0" && ch <= "9") || ch === undefined) {
            setNumArg(value);
        }
    }

    const [argValue, setArgValue] = useState("");

    async function trueOrFalseInputHandler(event) {
        const value = event.target.value;
        if (value === "1" || value === "0" || value === undefined || value === "") {
            setArgValue( value );
        }
    }

    const [output, setOutput] = useState("-");
    const [err, setErr] = useState("");

    useEffect(() => {
        if( isOk && numArg.length !== 0 ) {
            invoke("get_remind_function", { func: inputValue, n: numArg, value : argValue === "1" })
            .then((message) => {
                setErr("");
                setOutput(message);
            }).catch((err) => {
                setErr(err);
            });
        }
        if(numArg.length === 0 || inputValue.length === 0) {
            setErr("");
            setOutput("-");
        }
    });


    return (
        <>
            <div className="grid grid-cols-2 grid-rows-1 gap-4 select-none">
                
                <div className="col-span-2">
                    <InputBooleanFunction isOk={isOk} handleInput={handleInput} inputValue={inputValue}/>
                </div>
                <div className="">
                    <input placeholder="True or False" maxLength="1" value={argValue} onInput={trueOrFalseInputHandler}></input>
                </div>
                <div className="">
                    <input placeholder="Введите номер аргумента" value={numArg} maxLength="1" onInput={handleArgsInput}></input>
                    <div className="px-3 py-1 text-slate-600"><MxArgRender value={mxArg}/></div>
                </div>
                
            </div>
            <div className="pt-8 text-center">
                <div className="text-slate-600 text-sm select-none">Остаточная:</div>
                <div className="pt-1 text-xl">
                    <OutputComponent err={err} output={output}/>
                </div>
            </div>
        </>
        
    )
}

export default Task2;