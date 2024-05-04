const VarButton = ( {val, index, userVals, onClick} ) => {
    const baseStyle = "block border-2 py-1 px-1 text-center select-none cursor-pointer transition-colors ";
    const greenStyle = baseStyle + "border-green-300 bg-green-200";
    const redStyle = baseStyle + "border-red-400 bg-red-300";


    return (
        <div className={ userVals[index] ? greenStyle : redStyle } onClick={onClick}>{val}</div>
    )
}

export default VarButton;