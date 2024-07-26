import { useEffect, useState } from "react";
import { Breadcrumb, Card, Col, Divider, Row, Typography } from 'antd';
import { Content } from "antd/es/layout/layout";
import { get } from "../services/api_service";

const { Meta } = Card;

const Home = () => {

    const [data, setData] = useState([]);


    useEffect(() => {
        get("/posts")
            .then(response => {
                setData(response.data);
            })
            .catch(err => {
                console.log(err)
            });
    }, []);


    return (
        <>
            <Content style={{ padding: '0 48px' }}>
                <Breadcrumb style={{ margin: '16px 0' }}>
                    <Breadcrumb.Item>
                        Home
                    </Breadcrumb.Item>
                </Breadcrumb>
            </Content>
            <Divider />
            <Typography.Title level={2}>
                All Posts
            </Typography.Title>

            <Content className='content'>
                <Row gutter={16} justify="space-between" align="top">
                    {data.map((post: any) => (
                    <Col xs={20} sm={8} span={8} style={{ marginBottom: "1em" }}>
                        <a href={"/post/" + post.id}>
                            <Card
                                cover={<img alt={post.image_path} src={"/images/" + post.image_path} />}
                            >
                                <Meta title={post.title} description={post.description.substring(0, 50)} />
                            </Card>
                        </a>
                    </Col>
                    ))}
                </Row>
            </Content>
        </>
    );
};

export default Home;
