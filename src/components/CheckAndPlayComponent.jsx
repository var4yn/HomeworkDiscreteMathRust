import PlayButton from "./PlayButton";
import CheckAnswerButton from "./CheckAnswerButton";

const CheckAndPlayComponent = ({checkCallback, playCallback}) => {
    return (

        <div className="grid gap-4 grid-flow-col">                
            <CheckAnswerButton onClick={checkCallback}/>
            <PlayButton onClick={playCallback}/>
        </div>

    )
}

export default CheckAndPlayComponent;