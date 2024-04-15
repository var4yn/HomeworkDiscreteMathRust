import Page from "../components/Page";

import { useLocation } from "react-router-dom";
import pages from "../utils/pages_info";

import transition from "../transition";

const TaskPage = () => {
    const {pathname} = useLocation();
    const local = pathname.slice(1);

    const description = pages[local].description;
    const title = pages[local].title;
    const Content = pages[local].component;
    return (
        <Page title={title} description={description}>
            <Content />
        </Page>
    )
}

export default transition(TaskPage);