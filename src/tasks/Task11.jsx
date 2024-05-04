import Task10and11base from "./Task10and11base";

import { getRandomFrom2to4 } from "../utils/util";

import { invoke } from "@tauri-apps/api/tauri";

async function generateFuncs() {
    const n = getRandomFrom2to4();
    let str = "";
    for(let i = 0; i < n; i++) {
        let result = await invoke("get_random_bool_func", { n : n + "" });
        if(str.length) {
            str += ", ";
        }
        str += result;
    }
    return str;
}

function handleInvoke(r, setVals, setMsg) {
    console.log(r);
    setVals(r[1]);
    setMsg(r[0]);

}

const Task11 = () => {
    return <Task10and11base isVisiableMessage={true} nameFunction={"match_functions_to_precomplete_classes"} generateFunc={generateFuncs} handleInvoke={handleInvoke}/>
}

export default Task11;

