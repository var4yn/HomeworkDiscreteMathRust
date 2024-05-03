import PlayButton from "../components/PlayButton";

import { useState, useEffect } from "react";

import { invoke } from "@tauri-apps/api/tauri";
import OutputComponent from "../components/OutputComponent";

const CheckButton = ( {children, onClick, select, current, is_play} ) => {
    const base = "shadow-sm min-w-min min-h-min\
                    border-2 block px-2\
                    rounded-md select-none cursor-pointer\
                    transition-all";

    const no_select = base + " bg-slate-200 border-slate-600 block px-2 rounded-md active:bg-slate-200 hover:border-red-400";
    const after_select = no_select + " hover:border-slate-600";

    const green = base + " bg-green-200 border-green-600 hover:border-green-600 active:bg-green-200";
    const red = base + " bg-red-200 border-red-600 hover:border-red-600 active:bg-red-200";

    const is_select = select === children;
    const is_correct = current === children;

    function get_style() {
        if( !is_play ) return no_select;
        if( select.length !== 0 && is_correct ) return green;
        if( is_select ) return is_correct ? green : red;
        if( select.length !== 0 ) return after_select;
        return no_select;
    }

    return (
        <button className={ get_style() } onClick={onClick}>
            {children}
        </button>
    )
}

function Task4() {

    const [current, setCurrent] = useState(null);
    const [select, setSelect] = useState("");
    const [output, setOutput] = useState("-");
    const [isPlay, setIsPlay] = useState(false);
    const [err, setErr] = useState("");

    function play() {
        invoke( "get_random_vector_func" )
        .then(msg => {
            setErr("");
            setCurrent(msg[1]);
            setOutput(msg[0]);
            setSelect("");
            setIsPlay(true);
        })
        .catch(err => {
            setErr(err);
        })
    }

    function handleClick(name) {
        if(!isPlay || select.length !== 0) return;
        setSelect(name);
    }


    const [buttons, setButtons] = useState([""]);



    // render
    useEffect(() => {
        invoke( "get_name_funcs" )
        .then( res => setButtons(res))
        .catch( err => setErr(err));
    }, []);

    return (
        <>
            <div className="grid grid-cols-4 gap-4 pt-6 font-serif">
                {buttons.map((name, i) => (
                    <CheckButton key={i} onClick={() => handleClick(name)}
                        select={select}
                        current={current}
                        is_play={isPlay}
                        >
                        {name}
                    </CheckButton>
                ))}
            </div>
            <div className="flex flex-col justify-center items-center font-mono pt-6 select-none">
                <div className="text-lg"><OutputComponent err={err} output={output}/></div>
            </div>
            <div className="pt-4">
                <PlayButton onClick={play}/>
            </div>
        </>
    )
}

export default Task4;