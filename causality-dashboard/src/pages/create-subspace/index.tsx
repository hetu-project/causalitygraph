import React from 'react';
import { Card, Form, Input, Button, Upload, message } from 'antd';
import { UploadOutlined } from '@ant-design/icons';
import { history } from '@umijs/max';

const { TextArea } = Input;

const CreateSubspace = () => {
  const [form] = Form.useForm();

  const onFinish = (values: any) => {
    console.log('Form values:', values);
    // TODO: 处理表单提交
    message.success('Subspace created successfully!');
    history.push('/vote');
  };

  return (
    <div className="p-6">
      <Card title="Create New Subspace" className="max-w-2xl mx-auto">
        <Form
          form={form}
          layout="vertical"
          onFinish={onFinish}
        >
          <Form.Item
            label="Subspace Name"
            name="name"
            rules={[{ required: true, message: 'Please input the subspace name!' }]}
          >
            <Input placeholder="Enter subspace name" />
          </Form.Item>

          <Form.Item
            label="Description"
            name="description"
            rules={[{ required: true, message: 'Please input the description!' }]}
          >
            <TextArea
              placeholder="Enter subspace description"
              rows={4}
            />
          </Form.Item>

          <Form.Item>
            <div className="flex justify-end gap-4">
              <Button onClick={() => history.push('/vote')}>
                Cancel
              </Button>
              <Button type="primary" htmlType="submit">
                Create Subspace
              </Button>
            </div>
          </Form.Item>
        </Form>
      </Card>
    </div>
  );
};

export default CreateSubspace; 