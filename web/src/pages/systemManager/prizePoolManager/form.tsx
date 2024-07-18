import React, { useContext } from 'react';
import { Form, Input, Button, Grid, Select } from '@arco-design/web-react';
import { GlobalContext } from '@/context';
import locale from './locale';
import useLocale from '@/utils/useLocale';
import { IconRefresh, IconSearch } from '@arco-design/web-react/icon';
import styles from './style/index.module.less';

const { Row, Col } = Grid;
const { useForm } = Form;

function SearchForm(props: {
  onSearch: (values: Record<string, any>) => void;
}) {
  const { lang } = useContext(GlobalContext);

  const t = useLocale(locale);
  const [form] = useForm();

  const handleSubmit = () => {
    const values = form.getFieldsValue();
    props.onSearch(values);
  };

  const handleReset = () => {
    form.resetFields();
    props.onSearch({});
  };

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
              label={t['searchTable.columns.pool_name']}
              field={'pool_name'}
            >
              <Input placeholder={t['searchForm.placeholder']} allowClear />
            </Form.Item>
          </Col>
          {/* <Col span={colSpan}>
            <Form.Item
              label={t['searchTable.columns.pool_type']}
              field={'pool_type'}
            >
              <Input placeholder={t['searchForm.placeholder']} allowClear />
            </Form.Item>
          </Col> */}
          <Col span={colSpan}>
            <Form.Item
              label={t['searchTable.columns.share_pool']}
              field={'share_pool'}
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
          {/* <Col span={colSpan}>
            <Form.Item
              label={t['searchTable.columns.strategy']}
              field={'strategy'}
            >
              <Input placeholder={t['searchForm.placeholder']} allowClear />
            </Form.Item>
          </Col> */}
          <Col span={colSpan}>
            <Form.Item
              label={t['searchTable.columns.pool_desc']}
              field={'pool_desc'}
            >
              <Input placeholder={t['searchForm.placeholder']} allowClear />
            </Form.Item>
          </Col>
          <Col span={colSpan}>
            <Form.Item label={t['searchTable.columns.status']} field={'status'}>
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
