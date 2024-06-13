import { Breadcrumb, Card, Col, Divider, Row, Typography } from 'antd';
import { Content } from "antd/es/layout/layout";
import { useEffect, useState } from "react";
import apiService from "../services/apiService";
import Meta from 'antd/es/card/Meta';

const Home = () => {

    const [data, setData] = useState([]);


    const Posts = () => {
        apiService.get("/posts")
            .then(response => {
                setData(response.data);
            })
            .catch(error => {

            })
    }

    useEffect(() => {
        Posts();
    }, [])


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
                    <Col span={8} style={{ marginBottom: "1em" }}>
                        <a href={"/post/" + post.id}>
                            <Card
                                cover={<img alt={post.image_path} src={"/images/" + post.image_path} />}
                            >
                                <Meta title={post.title} description={post.description} />
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
