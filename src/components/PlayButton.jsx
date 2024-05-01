
const PlayButton = ( {onClick} ) => {

    return (

        <div onClick={onClick} className="flex text-center justify-center border-2 border-black text-white hover:text-black bg-black hover:bg-white p-2 rounded-md select-none hover:bg-transparent transition-all cursor-pointer active:py-1 active:my-1">
            <p>играть!</p>
        </div>

    );
}

export default PlayButton;