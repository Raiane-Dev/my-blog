import { Breadcrumb, Divider, Row, Layout, Typography } from 'antd';
import { useEffect, useState } from "react";
import { get } from "../services/api_service";
import { PostPattern } from '../models/post';
import { RenderHTML } from './Render';
import { Link, useParams } from 'react-router-dom';

const { Content } = Layout;

const Article = () => {

    const [data, setData] = useState(PostPattern);
    const { post_id } = useParams();

    const Post = () => {
        get("/post/" + post_id)
            .then(response => {
                setData(response.data);
            })
            .catch(err => {
                console.log(err);
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
                        <Link to="/" relative="path">
                        posts 
                        </Link>
                         / {lower_str(data.title)}
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
