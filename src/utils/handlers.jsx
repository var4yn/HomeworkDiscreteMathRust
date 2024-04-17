import { useState } from "react";

import { __lg } from "./util";

export function useInputHandler( {mn} )  {

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

            const ok = value.length > mn && ( value.length & (value.length - 1) ) === 0;
            setOk( ok );
        }
    }
    
    return {
        inputValue, isOk, setOk, mxArg, handleInput
    }

}




export function useNumArgHandler() {

    const [numArg, setNumArg] = useState("");

    async function handleArgsInput(event) {
        const value = event.target.value;
        const ch = value[value.length - 1];
        if ( (ch >= "0" && ch <= "9") || ch === undefined) {
            setNumArg(value);
        }
    }

    return {
        numArg, handleArgsInput
    }


}