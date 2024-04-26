
const PlayButton = ( {onClick} ) => {

    return (

        <div onClick={onClick} className=" border-2 border-black text-white hover:text-black bg-black hover:bg-white p-2 rounded-md select-none hover:bg-transparent transition-all duration-[175] cursor-pointer active:py-1 active:my-1">
            играть!
        </div>

    );
}

export default PlayButton;