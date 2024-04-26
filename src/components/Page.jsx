import Header from "./Header";

import { Link } from "react-router-dom";

const Page = ( { title, description, children, isMain } ) => {

    return (
        <div className=" min-h-screen max-h-full flex-col flex font-mono bg-slate-100">
            <Header title={title}/>
            <div className="px-7 lg:px-28 select-none text-center text-slate-400 text-[14px] lg:text-[16px]">
                {description}
            </div>
            <div className="flex-auto flex flex-col justify-center items-center">
                {/* content */}
                {children}
            </div>
            { isMain || <div className="flex justify-center m-8 text-sm">
                <Link to ="/"><button>назад</button></Link>
            </div>}
        </div>

    )
}

export default Page;