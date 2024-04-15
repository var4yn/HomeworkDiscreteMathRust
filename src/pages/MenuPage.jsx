import Page from "../components/Page";
import buttons from "../utils/buttons";
import { Link } from "react-router-dom";
import transition from "../transition";

const MenuPage = () => {
    return (
        <Page title="Menu" isMain={true}>
            <div className="grid gap-8 grid-cols-3 lg:gap-x-16">
                
                {
                    buttons.map((it, index) => (
                        <Link key={index} to={it.link}>
                            <button key={index}>{it.title}</button>
                        </Link>
                    ))
                }
            </div>
        </Page>
    )
}

export default transition(MenuPage);