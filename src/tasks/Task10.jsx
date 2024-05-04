import { invoke } from "@tauri-apps/api/tauri";

import { getRandomFrom2to4 } from "../utils/util";

import Task10and11base from "./Task10and11base";

async function generateFunc() {
    const n = getRandomFrom2to4();
    let result = await invoke("get_random_bool_func", { n : n + "" });
    return result;
}

function handleInvoke(r, setVals, setMsg) {
    setVals(r);
}

const Task10 = () => {
    return <Task10and11base nameFunction={"match_function_to_precomplete_classes"} generateFunc={generateFunc} handleInvoke={handleInvoke}/>
}


export default Task10;