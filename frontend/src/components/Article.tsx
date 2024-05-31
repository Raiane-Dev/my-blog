import { Breadcrumb, Divider, Row, Typography } from 'antd';
import { Content } from "antd/es/layout/layout";
import { useEffect, useState } from "react";
import apiService from "../services/apiService";
import { PostPattern } from '../models/post';
import { RenderHTML } from './Render';

const Article = () => {

    const [data, setData] = useState(PostPattern);


    const Post = () => {
        apiService.get("/post/1")
            .then(response => {
                setData(response.data);
            })
            .catch(error => {

            })
    }

    const lower_str = (str: any) => {
        return str.toLowerCase().replace(/\s+/g, '');
    }

    useEffect(() => {
        Post();
    }, [])


    return (
        <>
            <Content style={{ padding: '0 48px' }}>
                <Breadcrumb style={{ margin: '16px 0' }}>
                    <Breadcrumb.Item>
                        post / {lower_str(data.title)}
                    </Breadcrumb.Item>
                </Breadcrumb>
            </Content>
            <Divider />

            <Content className='content'>
            <Typography.Title level={2}>
                {data.title}
            </Typography.Title>

                <Row gutter={16}>
                    <Typography.Paragraph>
                        <RenderHTML html={data.body} />
                    </Typography.Paragraph>
                </Row>
            </Content>
        </>
    );
};

export default Article;
