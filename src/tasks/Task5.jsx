import { useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";

import CheckAndPlayComponent from "../components/CheckAndPlayComponent";
import OutputComponent from "../components/OutputComponent";

import { getRandomFrom2to4 } from "../utils/util";

const VarButton = ( {val, index, userVals, onClick} ) => {
    const baseStyle = "block border-2 py-1 px-1 text-center select-none cursor-pointer transition-colors ";
    const greenStyle = baseStyle + "border-green-300 bg-green-200";
    const redStyle = baseStyle + "border-red-400 bg-red-300";


    return (
        <div className={ userVals[index] ? greenStyle : redStyle } onClick={onClick}>{val}</div>
    )
}

const Indicator = ( { isCheck, index, vals } ) => {        

    function getStyle() {
        if( isCheck && vals[index] !== true ) {
            return "bg-red-500 p-1";
        }
        return "bg-green-500 p-1";
    }

    return (
        <div className={`transition-colors ${getStyle()}`}>
        </div>
    )

}

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
            
            //console.log(r);
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