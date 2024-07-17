import React, { useContext, useEffect } from 'react';
import { Form, Input, Button, Grid, Select } from '@arco-design/web-react';
import { GlobalContext } from '@/context';
import locale from './locale';
import useLocale from '@/utils/useLocale';
import { IconRefresh, IconSearch } from '@arco-design/web-react/icon';
import styles from './style/index.module.less';
import { getResourceList } from '@/api/resource';
import { API_HTTP_METHOD } from './constants';

const { Row, Col } = Grid;
const { useForm } = Form;

function SearchForm(props: {
  onSearch: (values: Record<string, any>) => void;
}) {
  const { lang } = useContext(GlobalContext);

  const t = useLocale(locale);
  const [form] = useForm();

  const [resourceTree, setResourceTree] = React.useState([]);

  const handleSubmit = () => {
    const values = form.getFieldsValue();
    props.onSearch(values);
  };

  const handleReset = () => {
    form.resetFields();
    props.onSearch({});
  };

  useEffect(() => {
    getResourceList().then((res) => {
      const { success, data } = res.data;
      if (success) {
        setResourceTree(
          data.map((item) => {
            return {
              label: item.resource_name,
              value: item.id,
            };
          })
        );
      }
    });
  }, []);

  const colSpan = lang === 'zh-CN' ? 8 : 12;

  return (
    <div className={styles['search-form-wrapper']}>
      <Form
        form={form}
        className={styles['search-form']}
        labelAlign="left"
        labelCol={{ span: 5 }}
        wrapperCol={{ span: 19 }}
      >
        <Row gutter={24}>
          <Col span={colSpan}>
            <Form.Item
              required
              label={t['searchTable.columns.parent_id']}
              field={'parent_id'}
            >
              <Select
                showSearch
                placeholder={t['searchForm.placeholder']}
                options={resourceTree}
                allowClear
                filterOption={(inputValue, option) =>
                  option.props.children.indexOf(inputValue) >= 0
                }
              />
            </Form.Item>
          </Col>

          <Col span={colSpan}>
            <Form.Item
              required
              label={t['searchTable.columns.resource_name']}
              field={'resource_name'}
            >
              <Input placeholder={t['searchForm.placeholder']} allowClear />
            </Form.Item>
          </Col>
          <Col span={colSpan}>
            <Form.Item
              required
              label={t['searchTable.columns.resource_code']}
              field={'resource_code'}
            >
              <Input placeholder={t['searchForm.placeholder']} allowClear />
            </Form.Item>
          </Col>
          <Col span={colSpan}>
            <Form.Item
              required
              label={t['searchTable.columns.resource_type']}
              field={'resource_type'}
            >
              <Select
                placeholder={t['searchForm.placeholder']}
                options={[
                  {
                    label: '菜单',
                    value: 1,
                  },
                  {
                    label: '功能',
                    value: 2,
                  },
                ]}
                allowClear
              />
            </Form.Item>
          </Col>
          <Col span={colSpan}>
            <Form.Item
              required
              defaultValue={'false'}
              label={t['searchTable.columns.resource_root']}
              field={'resource_root'}
            >
              <Select
                placeholder={t['searchForm.placeholder']}
                options={[
                  {
                    label: t['searchTable.columns.yes'],
                    value: 'true',
                  },
                  {
                    label: t['searchTable.columns.no'],
                    value: 'false',
                  },
                ]}
                allowClear
              />
            </Form.Item>
          </Col>
          <Col span={colSpan}>
            <Form.Item
              required
              defaultValue={'false'}
              label={t['searchTable.columns.resource_action']}
              field={'resource_action'}
            >
              <Select
                placeholder={t['searchForm.placeholder']}
                options={[
                  {
                    label: t['searchTable.columns.yes'],
                    value: 'true',
                  },
                  {
                    label: t['searchTable.columns.no'],
                    value: 'false',
                  },
                ]}
                allowClear
              />
            </Form.Item>
          </Col>
          <Col span={colSpan}>
            <Form.Item label={t['searchTable.columns.url']} field={'url'}>
              <Input placeholder={t['searchForm.placeholder']} allowClear />
            </Form.Item>
          </Col>
          <Col span={colSpan}>
            <Form.Item
              required
              defaultValue={'true'}
              label={t['searchTable.columns.status']}
              field={'status'}
            >
              <Select
                placeholder={t['searchForm.placeholder']}
                options={[
                  {
                    label: t['searchForm.enable'],
                    value: 'true',
                  },
                  {
                    label: t['searchForm.disable'],
                    value: 'false',
                  },
                ]}
                allowClear
              />
            </Form.Item>
          </Col>
          <Col span={colSpan}>
            <Form.Item
              label={t['searchTable.columns.api_path']}
              field={'api_path'}
            >
              <Input placeholder={t['searchForm.placeholder']} allowClear />
            </Form.Item>
          </Col>
          <Col span={colSpan}>
            <Form.Item
              label={t['searchTable.columns.api_http_method']}
              field={'api_http_method'}
            >
              <Select
                placeholder={t['searchForm.placeholder']}
                options={API_HTTP_METHOD.map((item) => {
                  return {
                    label: item,
                    value: item,
                  };
                })}
                allowClear
              />
            </Form.Item>
          </Col>
          <Col span={colSpan}>
            <Form.Item label={t['searchTable.columns.role']} field={'role'}>
              <Input placeholder={t['searchForm.placeholder']} allowClear />
            </Form.Item>
          </Col>
          <Col span={colSpan}>
            <Form.Item
              label={t['searchTable.columns.resource_desc']}
              field={'resource_desc'}
            >
              <Input placeholder={t['searchForm.placeholder']} allowClear />
            </Form.Item>
          </Col>
        </Row>
      </Form>
      <div className={styles['right-button']}>
        <Button type="primary" icon={<IconSearch />} onClick={handleSubmit}>
          {t['searchTable.form.search']}
        </Button>
        <Button icon={<IconRefresh />} onClick={handleReset}>
          {t['searchTable.form.reset']}
        </Button>
      </div>
    </div>
  );
}

export default SearchForm;
