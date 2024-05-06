import { useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";

import OutputComponent from "../components/OutputComponent";
import CheckAndPlayComponent from "../components/CheckAndPlayComponent";

import { getRandomFrom2to4 } from "../utils/util";
import OkOrInvalidComponent from "../components/OkOrInvalidComponent";

function Task6and7base( { nameFunction } ) {
    const [expression, setExpression] = useState("");

    const [output, setOutput] = useState("-");
    const [err, setErr] = useState("");

    const [userErr, setUserErr] = useState("");
    const [result, setResult] = useState(true);

    function handleInput(event) {
        setExpression( event.target.value );
    }

    async function play() {
        const n = getRandomFrom2to4();

        try {
            const result = await invoke("get_random_bool_func", { n: n + "" })
            setErr(err);
            setOutput(result);
        } catch(err) {
            setErr(err);
        }
        
    }
    // check expression
    async function check() {
        invoke( nameFunction, { function: output, expression: expression } )
        .then(res => {
            setUserErr("");            
            setResult(res);
        })
        .catch(err => setUserErr(err));
    }

    return (
        <>
            <div className="text-4xl text-center">
                <OutputComponent err={err} output={output}/>
            </div>
            <div className="grid grid-flow-row gap-3 justify-items-center lg:pt-6">
                <div className="">
                    <textarea value={expression} onInput={handleInput} maxLength={65} className="max-h-[200px] min-h-[200px] min-w-[300px] p-2 resize-none border-2 border-slate-800"></textarea>
                </div>
                
                <div className="text-2xl">
                    <OutputComponent err={userErr} output={<OkOrInvalidComponent condition={result}/>}/>
                </div>
                <div className="pt-4">
                    <CheckAndPlayComponent playCallback={play} checkCallback={check}/>
                </div>
            </div>
        </>
    )
}

export default Task6and7base;
