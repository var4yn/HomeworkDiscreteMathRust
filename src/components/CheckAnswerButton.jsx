
const CheckAnswerButton = ({onClick}) => {
    return (
        <div onClick={onClick} className=" active:bg-amber-600 transition-all cursor-pointer text-center border-2 border-transparent justify-center items-baseline flex content-baseline select-none text-white p-2 rounded-md  bg-amber-500">
            <p>проверить</p>
        </div>
    )
}

export default CheckAnswerButton;