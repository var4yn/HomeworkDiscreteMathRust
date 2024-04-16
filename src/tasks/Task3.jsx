
const Task3 = () => {

    return (
        <>
            <div className="grid grid-cols-2 grid-rows-1 gap-4 select-none">
                
                <div className="">
                    <input placeholder="Нулевая остаточная"maxLength="16"></input>
                    <div className="px-3 py-1">ok</div>
                </div>
                <div className="">
                    <input placeholder="Единичная остаточная" maxLength="16"></input>
                </div>
                <div className="">
                    <input placeholder="Номер аргумента" maxLength="1"></input>
                </div>
                
            </div>
            <div className="pt-8 text-center">
                <div className="text-slate-600 text-sm select-none">Функция:</div>
                <div className="pt-1 text-xl">
                    "msg"
                </div>
            </div>
        </>
    )
}

export default Task3;