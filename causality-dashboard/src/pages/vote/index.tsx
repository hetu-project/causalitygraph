import React from "react";
import { Card, Button, Statistic } from "antd";
import { FileTextOutlined, MessageOutlined, PlusOutlined } from "@ant-design/icons";
import { history } from '@umijs/max';
import './index.less';

interface SubspaceCardProps {
  id: string;
  image: string;
  name: string;
  description: string;
  proposals: number;
  posts: number;
}

const SubspaceCard: React.FC<SubspaceCardProps> = ({
  id,
  image,
  name,
  description,
  proposals,
  posts,
}) => {
  const handleEnterSubspace = () => {
    history.push(`/governance/${id}`);
  };

  return (
    <Card className="subspace-card">
      {/* 上部分：水平布局 */}
      <div className="card-content">
        {/* 左侧图片 */}
        <div className="card-image">
          <img
            src={image}
            alt={name}
            className="image"
          />
        </div>
        
        {/* 中间文字部分 */}
        <div className="card-info">
          <h2 className="card-title">{name}</h2>
          <p className="card-description">{description}</p>
        </div>
        
        {/* 右侧统计数据 */}
        <div className="card-stats">
          <Statistic
            title="Proposals"
            value={proposals}
            prefix={<FileTextOutlined />}
          />
          <Statistic
            title="Posts"
            value={posts}
            prefix={<MessageOutlined />}
          />
        </div>
      </div>

      {/* 下部分：按钮 */}
      <div className="card-footer">
        <Button type="primary" size="large" className="enter-button" onClick={handleEnterSubspace}>
          Enter Subspace
        </Button>
      </div>
    </Card>
  );
};

const Vote = () => {
  // 示例数据
  const subspaces = [
    {
      id: "ai-research",
      image: "/image.png",
      name: "AI Research",
      description: "A community focused on artificial intelligence research and development, sharing the latest breakthroughs and discussing future directions.",
      proposals: 56,
      posts: 789,
    },
    {
      id: "blockchain",
      image: "/image.png",
      name: "Blockchain",
      description: "Exploring blockchain technology, cryptocurrencies, and decentralized applications.",
      proposals: 78,
      posts: 456,
    },
    {
      id: "web-dev",
      image: "/image.png",
      name: "Web Development",
      description: "A community for web developers to share knowledge and best practices.",
      proposals: 90,
      posts: 567,
    },
  ];

  const handleCreateSubspace = () => {
    history.push('/create-subspace');
  };

  return (
    <div className="vote-page">
      <div className="page-header">
        <h1 className="page-title">Subspaces</h1>
        <Button 
          type="primary" 
          icon={<PlusOutlined />}
          onClick={handleCreateSubspace}
          className="create-button"
        >
          Create Subspace
        </Button>
      </div>
      <div className="subspace-grid">
        {subspaces.map((subspace) => (
          <SubspaceCard key={subspace.id} {...subspace} />
        ))}
      </div>
    </div>
  );
};

export default Vote;