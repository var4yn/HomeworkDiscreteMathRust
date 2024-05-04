const Indicator = ( { isCheck, index, vals } ) => {        

    function getStyle() {
        if( isCheck && vals[index] !== true ) {
            return "bg-red-500 p-1";
        }
        return "bg-green-500 p-1";
    }

    return (
        <div className={`transition-colors ${getStyle()}`}>
        </div>
    )

}

export default Indicator;