import { Row } from 'antd';
import { useEffect, useState } from 'react';
import { Content } from "antd/es/layout/layout";

const NotFound = () => {

    const [xAxis, setXAxis] = useState(0);
    const [yAxis, setYAxis] = useState(0);

    useEffect(() => {
        const handleMouseMove = (event: any) => {
            const pageX = window.innerWidth;
            const pageY = window.innerHeight;
            const mouseY = event.pageY;
            const yAxis = ((pageY / 2 - mouseY) / pageY) * 300;
            const mouseX = event.pageX / -pageX;
            const xAxis = -mouseX * 100 - 100;

            setXAxis(xAxis);
            setYAxis(yAxis);
        };

        document.addEventListener('mousemove', handleMouseMove);

        return () => {
            document.removeEventListener('mousemove', handleMouseMove);
        };
    }, []);

    return (
        <>
            <Content className='component-notfound'>
                <Row className="box">
                    <Row className="box__ghost">
                        <Row className="symbol"></Row>
                        <Row className="symbol"></Row>
                        <Row className="symbol"></Row>
                        <Row className="symbol"></Row>
                        <Row className="symbol"></Row>
                        <Row className="symbol"></Row>

                        <Row className="box__ghost-container">
                            <Row className="box__ghost-eyes">
                                <Row className="box__eye-left" style={{ transform: `translate(${xAxis}%, -${yAxis}%)` }}></Row>
                                <Row className="box__eye-right" style={{ transform: `translate(${xAxis}%, -${yAxis}%)` }}></Row>
                            </Row>
                            <Row className="box__ghost-bottom">
                                <Row></Row>
                                <Row></Row>
                                <Row></Row>
                                <Row></Row>
                                <Row></Row>
                            </Row>
                        </Row>
                        <Row className="box__ghost-shadow"></Row>
                    </Row>

                    <Row className="box__description">
                        <Row className="box__description-container">
                            <Row className="box__description-title">Ops! parece que essa página não existe :(</Row>
                        </Row>

                        <a href="/" className="box__button">Voltar</a>

                    </Row>

                </Row>
            </Content>
        </>
    );
};

export default NotFound;
