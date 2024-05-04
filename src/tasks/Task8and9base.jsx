import { useEffect, useState } from "react";
import InputBooleanFunction from "../components/InputBooleanFunction";
import { useInputHandler } from "../utils/handlers";

import { invoke } from "@tauri-apps/api/tauri";

const Task8and9base = ({nameFunction, mn}) => {

    const [output, setOutput] = useState();

    const {
        inputValue, isOk, handleInput
    } = useInputHandler( {mn: mn ? mn : 1} );

    useEffect(() => {
        if(!isOk) {
            setOutput("");
            return;
        }

        invoke( nameFunction, { function: inputValue } )
        .then(res => {
            setOutput(res);
        })
        .catch(err => {
            setOutput(err);
        });

    }, [isOk, inputValue]);

    return (
        <>
            <div className="pt-8 grid grid-flow-row justify-items-center">
                <div className="flex flex-col min-w-[325px]">
                    <InputBooleanFunction isOk={isOk} handleInput={handleInput} inputValue={inputValue}/>
                </div>
                <div className="mt-4 overflow-y-auto bg-white w-[525px] h-[150px] text-center border-2 border-slate-500 p-0.5">
                    {output}
                </div>
            </div>
        </>
    )

}

export default Task8and9base;