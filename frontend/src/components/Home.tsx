import { Breadcrumb, Card, Divider, Row, Typography } from 'antd';
import { Content } from "antd/es/layout/layout";
import { useEffect, useState } from "react";
import apiService from "../services/apiService";
import Meta from 'antd/es/card/Meta';

const Home = () => {

    const [data, setData] = useState([]);


    const Posts = () => {
        apiService.get("/covers")
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
                <Row gutter={16}>
                    {data.map((post: any) => (
                        <a href={"/post/" + post.post_id}>
                            <Card
                                style={{ width: 240 }}
                                cover={<img alt={post.image_path} src={post.image_path} />}
                            >
                                <Meta title={post.headline} description={post.description} />
                            </Card>
                        </a>
                    ))}
                </Row>
            </Content>
        </>
    );
};

export default Home;
