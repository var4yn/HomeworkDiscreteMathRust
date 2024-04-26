import PlayButton from "../components/PlayButton";

import { useState, useEffect } from "react";

import { invoke } from "@tauri-apps/api/tauri";

const CheckButton = ( {children, onClick, id} ) => {
    return (
        <button id={id} className="shadow-sm min-w-min min-h-min bg-slate-200 border-2 border-slate-800 block px-2 rounded-md select-none cursor-pointer hover:bg-red-200 hover:border-red-600 transition-all" onClick={onClick}>{children}</button>
    )
}

function Task4() {

    function play() {
        setSelect(-1);
        console.log("play!");
    }

    const [buttons, setButtons] = useState([""]);

    function click(value) {
        console.log(value);
    }

    useEffect(() => {
        invoke( "get_name_funcs" ).then( res => setButtons(res));
      }, []);

    return (
        <>
            <div className="grid grid-cols-4 gap-4 pt-6 font-serif">
                {buttons.map((btn, i) => (
                    <CheckButton key={i} onClick={() => click(btn)}>
                        {btn}
                    </CheckButton>
                ))}
            </div>
            <div className="flex flex-col justify-center items-center font-mono pt-20">
                <div>Вектор:</div>
                <div>-</div>
            </div>
            <div className="pt-4">
                <PlayButton onClick={play}/>
            </div>
        </>
    )
}

export default Task4;