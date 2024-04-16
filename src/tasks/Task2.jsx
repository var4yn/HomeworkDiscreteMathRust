import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

const OkRender = () => {
    return (
        <div className="text-green-600">
            ok
        </div>
    )
}

const InvalidRender = () => {
    return (
        <div className="text-red-600">
            invalid
        </div>
    )
}

const MxArgRender = ({value}) => {
    const message = `valid max value argument: ${value}`;
    return (
        <div>
            {message}
        </div>
    )
}

const ErrorRender = ( {err} ) => {

    return (
        <div className="text-red-500">
            {`Ошибка: ${err}`}
        </div>
    )
}

const OutputRender = ({output}) => {
    return (
        <div>
            {output}
        </div>
    )
}

const __lg = (n) => {
    let cnt = 0;
    let r = 1;
    while( r * 2 <= n ) {
        r *= 2;
        cnt += 1;
    }
    return cnt;
}

const Task2 = () => {
    const [inputValue, setInputValue] = useState("");
    const [isOk, setOk] = useState(false);
    
    const [mxArg, setMxArg] = useState(0);

    async function handleInput(event) {
        const value = event.target.value;
        const ch = value[value.length - 1];
        if (ch === "1" || ch === "0" || ch === undefined) {
            setInputValue(value);            
            const mx = __lg(value.length);
            setMxArg(mx);

            const ok = value.length > 1 && ( value.length & (value.length - 1) ) === 0;
            setOk( ok );
        }
    }

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
                    <input placeholder="Введите булевую функцию" value={inputValue} maxLength="32" onInput={handleInput}></input>
                    <div className="px-3 py-1">{isOk === false ? <InvalidRender/> : <OkRender/>}</div>
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
                    { err.length > 0 ? <ErrorRender err={err}/> : <OutputRender output={output}/> }
                </div>
            </div>
        </>
        
    )
}

export default Task2;