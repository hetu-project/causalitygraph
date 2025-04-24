import { Outlet } from '@umijs/max';
import { Layout, Menu } from 'antd';
import { Link } from '@umijs/max';
import { HomeOutlined, CalculatorOutlined, CheckSquareOutlined, AlignLeftOutlined, AppstoreOutlined } from '@ant-design/icons';

const { Header, Content, Sider } = Layout;

export default function CustomLayout() {
  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Header style={{ display: 'flex', alignItems: 'center', background: '#fff', padding: '0 24px' }}>
        <div style={{ fontSize: '20px', fontWeight: 'bold' }}>Causality Graph</div>
      </Header>
      <Layout>
        <Sider width={200} style={{ background: '#fff' }}>
          <Menu
            mode="inline"
            defaultSelectedKeys={['1']}
            style={{ height: '100%', borderRight: 0 }}
          >
            <Menu.Item key="1" icon={<HomeOutlined />}>
              <Link to="/">Home</Link>
            </Menu.Item>
            <Menu.Item key="2" icon={<CalculatorOutlined />}>
              <Link to="/compute">Compute</Link>
            </Menu.Item>
            <Menu.Item key="3" icon={<CheckSquareOutlined />}>
              <Link to="/vote">Vote</Link>
            </Menu.Item>
            <Menu.Item key="4" icon={<AlignLeftOutlined />}>
              <Link to="/alignment">Alignment</Link>
            </Menu.Item>
            <Menu.Item key="5" icon={<AppstoreOutlined />}>
              <Link to="/subspace">Subspace</Link>
            </Menu.Item>
          </Menu>
        </Sider>
        <Layout style={{ padding: '24px' }}>
          <Content
            style={{
              padding: 24,
              margin: 0,
              minHeight: 280,
              background: '#fff',
            }}
          >
            <Outlet />
          </Content>
        </Layout>
      </Layout>
    </Layout>
  );
} 