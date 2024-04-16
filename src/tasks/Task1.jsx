import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

const get_buttons = () => {
    const n = 5;
    let btns = []
    for(let i = 1; i <= n; i++) {
        btns.push(i + "");
    }
    return btns;
}

const Task1 = () => {
    const [booleanFunction, setBooleanFunction] = useState("-");
    
    async function onClick(n) {
        setBooleanFunction( await invoke("get_random_bool_func", { n }) );
    }
    

    return (
        <>
            <div className="mb-5">
                <div className="font-semibold text-slate-600 select-none">Выберите n:</div>
                <div className="grid mt-4 gap-3 grid-cols-5">
                    {
                        get_buttons().map((it, index) => (
                            <button key={index} onClick={() => onClick(it)}>{it}</button>
                        ))
                    }
                </div>
            </div>

            <div className="text-center break-all px-16">
                {booleanFunction}
            </div>
        </>
    )
}

export default Task1;
