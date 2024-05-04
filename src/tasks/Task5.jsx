import { useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";

import CheckAndPlayComponent from "../components/CheckAndPlayComponent";
import OutputComponent from "../components/OutputComponent";
import VarButton from "../components/VarButton";
import Indicator from "../components/Indicator";

import { getRandomFrom2to4 } from "../utils/util";

function get_vars( n ) {
    let btns = [];
    for(let i = 1; i <= n; i++) {
        btns.push(`x${i}`);
    }
    return btns;
}

function init_user_vals( n ) {
    let usVals = [];
    while(n  --> 0) {
        usVals.push(true);
    }
    return usVals;

}

// Рандомно выбрать функцию -> вычислить фиктивные переменные -> дать юзеру выбрать фиктивные переменные
function Task5() {

    const [vars, setVars] = useState(["-"]);
    const [vals, setVals] = useState([true]);

    const [userVals, setUserVals] = useState([true]);

    const [output, setOutput] = useState("-");
    const [err, setErr] = useState("");

    const [isCheck, setIsCheck] = useState(true);

    

    async function play() {
        const n = getRandomFrom2to4();
        invoke("get_random_bool_func", { n: n + "" })
        .then(async msg => {
            setIsCheck(false);
            setVars( get_vars(n) );
            setUserVals( init_user_vals(n) );
            const r = await invoke("get_dummy_variable", { function: msg });
            setVals(r);
            setOutput(msg);
            setErr("");
        })
        .catch(err => {
            setErr(err);
        });
    }

    function check() {
        setIsCheck(true);
    }

    const [f, setF] = useState(0);
    function toggleClick( index ) {
        if(isCheck) return;
        userVals[index] = !userVals[index];
        setF(!f);
    }

    return (
        <>
            <div className="">
                <div className="text-4xl select-none flex justify-center">
                    <OutputComponent err={err} output={output}/>
                </div>
                <div className="grid mt-4 gap-3 grid-flow-col">
                    {
                        vars.map((val, i) => (
                            <div key={i} className="grid gap-2 grid-flow-row">
                             <VarButton val={val} index={i} userVals={userVals} onClick={() => toggleClick(i)}/>
                             <Indicator isCheck={isCheck} vals={vals} index={i}/>
                            </div>
                        ))
                    }
                </div>
                
            </div>
            <div className="pt-8">
                <CheckAndPlayComponent checkCallback={check} playCallback={play}/>
            </div>
            
        </>
    )

}

export default Task5;