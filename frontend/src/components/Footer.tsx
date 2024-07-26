import { Layout } from 'antd';

const Footer = () => {

    return (
        <>
            <Layout.Footer style={{ textAlign: 'center' }}>Rai Dev ©{new Date().getFullYear()}</Layout.Footer>
        </>
    );
};

export default Footer;
