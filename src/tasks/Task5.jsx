import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";

import PlayButton from "../components/PlayButton";
import CheckAnswerButton from "../components/CheckAnswerButton";

const VarButton = ( {val, onClick} ) => {

    


    return (
        <div className="block border-2 py-1 px-1 border-black text-center select-none cursor-pointer" onClick={onClick}>{val}</div>
    )
}



function get_vars( n ) {
    let btns = [];
    for(let i = 1; i <= n; i++) {
        btns.push(`x${i}`);
    }

    return btns;
}

// Рандомно выбрать функцию -> вычислить фиктивные переменные -> дать юзеру выбрать фиктивные переменные
function Task5() {

    const [value, setValue] = useState("-");
    const [vars, setVars] = useState(["-"]);
    const [vals, setVals] = useState([]);

    const getRandom = () => {
        return Math.floor( Math.random()*3 ) + 2;
    }

    async function play() {
        const n = getRandom();
        invoke("get_random_bool_func", { n: n + "" })
        .then(async msg => {
            setVars( get_vars(n) );
            const r = await invoke("get_dummy_variable", { function: msg });
            setVals(r);
            setValue(msg);
        })
        .catch(err => {
            console.log(err);
        });
    }

    return (
        <>
            <div className="">
                <div className="text-4xl select-none flex justify-center">
                    {value}
                </div>
                <div className="grid mt-4 gap-3 grid-flow-col">
                    {
                        vars.map((val, i) => (
                            <VarButton key={i} val={val}/>
                        ))
                    }
                </div>
                
            </div>
            
            <div className="pt-8 grid gap-4 grid-flow-col">                
                <CheckAnswerButton/>
                <PlayButton onClick={play}/>
            </div>
        </>
    )

}

export default Task5;