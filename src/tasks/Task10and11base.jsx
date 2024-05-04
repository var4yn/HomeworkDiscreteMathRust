import { invoke } from "@tauri-apps/api/tauri";

import { useState } from "react";

import OutputComponent from "../components/OutputComponent";
import CheckAndPlayComponent from "../components/CheckAndPlayComponent";

import VarButton from "../components/VarButton";
import Indicator from "../components/Indicator";

const MessageComponent = ( {isComplete, isCheck} ) => {

    return (
        <>
            <div className="mt-4">
                {isCheck ? isComplete ? "Система полная!" : "-" : "-"}
            </div>
        </>
    )
}

const Task10and11base = ( {nameFunction, generateFunc, handleInvoke, isVisiableMessage} ) => {
    
    const [output, setOutput] = useState("-");
    const [err, setErr] = useState("");

    const [vals, setVals] = useState([true, true, true, true, true]);

    const [userVals, setUserVals] = useState([true, true, true, true, true]);
    const [isCheck, setIsCheck] = useState(true);

    const [msg, setMsg] = useState(false);

    const precomplete_classes = [ "T0", "T1", "S", "M", "L" ];

    async function check() {
        setIsCheck(true);
    }

    async function play() {
        
        try {
            const func = await generateFunc();
            console.log(func);
            setIsCheck(false);
            setUserVals( [true, true, true, true, true] );
            const r = await invoke(nameFunction, { function: func });

            handleInvoke(r, setVals, setMsg);

            setOutput(func);
            setErr("");
        } catch(err) {
            setErr(err);
        }
    }

    const [f, setF] = useState(0);
    function toggleClick( index ) {
        if(isCheck) return;
        userVals[index] = !userVals[index];
        setF(!f);
    }

    return (
        <>
            <div className="text-2xl lg:text-4xl select-none text-wrap w-[400px] text-center">
                <OutputComponent err={err} output={output}/>
            </div>
            <div className="grid grid-flow-row justify-items-center">
                <div className="grid mt-4 gap-3 grid-flow-col">
                    {
                        precomplete_classes.map((val, i) => (
                            <div key={i} className="grid gap-2 grid-flow-row">
                             <VarButton val={val} index={i} userVals={userVals} onClick={() => toggleClick(i)}/>
                             <Indicator isCheck={isCheck} vals={vals} index={i}/>
                            </div>
                        ))
                    }
                </div>
                <div className="mt-8">
                    <CheckAndPlayComponent checkCallback={check} playCallback={play}/>
                </div>
                {
                    isVisiableMessage && <MessageComponent isComplete={msg} isCheck={isCheck} setMsg={setMsg}/>
                }
            </div>
            
        </>
    )


}

export default Task10and11base;