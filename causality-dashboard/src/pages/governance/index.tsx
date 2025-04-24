import React, { useState } from 'react';
import { useParams } from '@umijs/max';
import { Card, Timeline, Typography, Button, Avatar, List, Input, Space, Drawer, Progress, Tag } from 'antd';
import { FileTextOutlined, MessageOutlined, UserOutlined, SendOutlined, RightOutlined } from '@ant-design/icons';

const { Title, Text, Paragraph } = Typography;
const { TextArea } = Input;

interface Proposal {
  id: string;
  title: string;
  content: string;
  author: string;
  timestamp: string;
  status: 'active' | 'passed' | 'rejected';
  votes: {
    for: number;
    against: number;
  };
  comments: Comment[];
  voters: Voter[];
}

interface Comment {
  id: string;
  author: string;
  content: string;
  timestamp: string;
}

interface Voter {
  id: string;
  name: string;
  vote: 'for' | 'against';
  timestamp: string;
  votingPower: number;
}

const Governance = () => {
  const { subspaceId } = useParams<{ subspaceId: string }>();
  const [selectedProposal, setSelectedProposal] = useState<Proposal | null>(null);
  const [newComment, setNewComment] = useState('');
  const [drawerVisible, setDrawerVisible] = useState(false);

  // 示例数据
  const proposals: Proposal[] = [
    {
      id: '1',
      title: 'Increase DAO Treasury Allocation',
      content: 'Proposal to increase the DAO treasury allocation from 10% to 15% to support more community initiatives.',
      author: 'Alice',
      timestamp: '2024-03-15 10:00',
      status: 'active',
      votes: {
        for: 120,
        against: 45,
      },
      comments: [
        {
          id: '1',
          author: 'Bob',
          content: 'I support this proposal as it will help fund more community projects.',
          timestamp: '2024-03-15 11:00',
        },
        {
          id: '2',
          author: 'Charlie',
          content: 'I think we should consider the impact on token holders first.',
          timestamp: '2024-03-15 12:00',
        },
      ],
      voters: [
        {
          id: '1',
          name: 'Alice',
          vote: 'for',
          timestamp: '2024-03-15 10:05',
          votingPower: 100,
        },
        {
          id: '2',
          name: 'Bob',
          vote: 'for',
          timestamp: '2024-03-15 10:10',
          votingPower: 50,
        },
        {
          id: '3',
          name: 'Charlie',
          vote: 'against',
          timestamp: '2024-03-15 10:15',
          votingPower: 30,
        },
      ],
    },
    // 可以添加更多提案
  ];

  const handleProposalClick = (proposal: Proposal) => {
    setSelectedProposal(proposal);
  };

  const handleCommentSubmit = () => {
    if (!newComment.trim()) return;
    
    // TODO: 处理评论提交
    console.log('New comment:', newComment);
    setNewComment('');
  };

  const totalVotes = selectedProposal ? 
    selectedProposal.votes.for + selectedProposal.votes.against : 0;
  const forPercentage = totalVotes ? 
    Math.round((selectedProposal?.votes.for || 0) / totalVotes * 100) : 0;
  const againstPercentage = totalVotes ? 
    Math.round((selectedProposal?.votes.against || 0) / totalVotes * 100) : 0;

  return (
    <div className="p-6">
      <div className="flex gap-6">
        {/* 左侧时间轴 */}
        <div className="w-80 flex-shrink-0">
          <Card title="Proposals Timeline" className="h-full">
            <Timeline
              items={proposals.map(proposal => ({
                color: proposal.status === 'active' ? 'blue' : 
                       proposal.status === 'passed' ? 'green' : 'red',
                children: (
                  <div 
                    className="cursor-pointer hover:bg-gray-50 p-2 rounded"
                    onClick={() => handleProposalClick(proposal)}
                  >
                    <Text strong>{proposal.title}</Text>
                    <br />
                    <Text type="secondary" className="text-sm">
                      {proposal.timestamp}
                    </Text>
                  </div>
                ),
              }))}
            />
          </Card>
        </div>

        {/* 中间提案详情 */}
        <div className="flex-1">
          {selectedProposal ? (
            <Card>
              <div className="mb-6">
                <Title level={2}>{selectedProposal.title}</Title>
                <Space className="mb-4">
                  <Avatar icon={<UserOutlined />} />
                  <Text>{selectedProposal.author}</Text>
                  <Text type="secondary">{selectedProposal.timestamp}</Text>
                </Space>
                <Paragraph>{selectedProposal.content}</Paragraph>
                
                {/* 投票统计 */}
                <div className="flex gap-8 my-6">
                  <div>
                    <Text type="success">For: {selectedProposal.votes.for}</Text>
                  </div>
                  <div>
                    <Text type="danger">Against: {selectedProposal.votes.against}</Text>
                  </div>
                </div>

                {/* 投票按钮 */}
                <Space className="mb-6">
                  <Button type="primary">Vote For</Button>
                  <Button danger>Vote Against</Button>
                </Space>

                {/* 评论区 */}
                <div className="mt-8">
                  <Title level={4}>Comments</Title>
                  <List
                    className="mb-6"
                    itemLayout="horizontal"
                    dataSource={selectedProposal.comments}
                    renderItem={comment => (
                      <List.Item>
                        <List.Item.Meta
                          avatar={<Avatar icon={<UserOutlined />} />}
                          title={
                            <Space>
                              <Text strong>{comment.author}</Text>
                              <Text type="secondary">{comment.timestamp}</Text>
                            </Space>
                          }
                          description={comment.content}
                        />
                      </List.Item>
                    )}
                  />

                  {/* 评论输入框 */}
                  <div className="mt-4">
                    <TextArea
                      value={newComment}
                      onChange={e => setNewComment(e.target.value)}
                      placeholder="Write a comment..."
                      rows={4}
                      className="mb-2"
                    />
                    <Button 
                      type="primary" 
                      icon={<SendOutlined />}
                      onClick={handleCommentSubmit}
                    >
                      Post Comment
                    </Button>
                  </div>
                </div>
              </div>
            </Card>
          ) : (
            <Card>
              <div className="text-center py-12">
                <FileTextOutlined className="text-4xl mb-4" />
                <Title level={4}>Select a proposal to view details</Title>
              </div>
            </Card>
          )}
        </div>

        {/* 右侧投票详情面板 */}
        {selectedProposal && (
          <div className="w-80 flex-shrink-0">
            <Card 
              title="Voting Details" 
              className="h-full"
              extra={
                <Button 
                  type="text" 
                  icon={<RightOutlined />} 
                  onClick={() => setDrawerVisible(true)}
                />
              }
            >
              <div className="space-y-4">
                <div>
                  <Text strong>Total Votes: {totalVotes}</Text>
                </div>
                <div>
                  <Text type="success">For ({forPercentage}%)</Text>
                  <Progress percent={forPercentage} status="active" strokeColor="#52c41a" />
                </div>
                <div>
                  <Text type="danger">Against ({againstPercentage}%)</Text>
                  <Progress percent={againstPercentage} status="active" strokeColor="#ff4d4f" />
                </div>
              </div>
            </Card>
          </div>
        )}
      </div>

      {/* 投票详情抽屉 */}
      <Drawer
        title="Voting Details"
        placement="right"
        onClose={() => setDrawerVisible(false)}
        open={drawerVisible}
        width={400}
      >
        {selectedProposal && (
          <div className="space-y-6">
            <div>
              <Title level={4}>Voters</Title>
              <List
                dataSource={selectedProposal.voters}
                renderItem={voter => (
                  <List.Item>
                    <List.Item.Meta
                      avatar={<Avatar icon={<UserOutlined />} />}
                      title={
                        <Space>
                          <Text strong>{voter.name}</Text>
                          <Tag color={voter.vote === 'for' ? 'success' : 'error'}>
                            {voter.vote === 'for' ? 'For' : 'Against'}
                          </Tag>
                        </Space>
                      }
                      description={
                        <Space direction="vertical" size={0}>
                          <Text type="secondary">{voter.timestamp}</Text>
                          <Text>Voting Power: {voter.votingPower}</Text>
                        </Space>
                      }
                    />
                  </List.Item>
                )}
              />
            </div>
          </div>
        )}
      </Drawer>
    </div>
  );
};

export default Governance; 