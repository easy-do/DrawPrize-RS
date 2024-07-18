import React, { useContext, useEffect } from 'react';
import { Form, Input, Button, Grid, Select } from '@arco-design/web-react';
import { GlobalContext } from '@/context';
import locale from './locale';
import useLocale from '@/utils/useLocale';
import { IconRefresh, IconSearch } from '@arco-design/web-react/icon';
import styles from './style/index.module.less';

const { Row, Col } = Grid;
const { useForm } = Form;

function SearchForm(props: {
  prizePoolId: number;
  onSearch: (values: Record<string, any>) => void;
}) {
  const { lang } = useContext(GlobalContext);

  const t = useLocale(locale);
  const [form] = useForm();

  const handleSubmit = () => {
    const values = form.getFieldsValue();
    values.pool_id = props.prizePoolId;
    props.onSearch(values);
  };

  const handleReset = () => {
    form.resetFields();
    props.onSearch({ pool_id: props.prizePoolId });
  };

  useEffect(() => {
    handleReset();
  }, [props.prizePoolId]);

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
              label={t['searchTable.columns.prize_name']}
              field={'prize_name'}
            >
              <Input placeholder={t['searchForm.placeholder']} allowClear />
            </Form.Item>
          </Col>
          <Col span={colSpan}>
            <Form.Item label={t['searchTable.columns.icon']} field={'icon'}>
              <Input placeholder={t['searchForm.placeholder']} allowClear />
            </Form.Item>
          </Col>
          <Col span={colSpan}>
            <Form.Item label={t['searchTable.columns.level']} field={'level'}>
              <Input
                type="number"
                placeholder={t['searchForm.placeholder']}
                allowClear
              />
            </Form.Item>
          </Col>
          <Col span={colSpan}>
            <Form.Item
              label={t['searchTable.columns.level_name']}
              field={'level_name'}
            >
              <Input placeholder={t['searchForm.placeholder']} allowClear />
            </Form.Item>
          </Col>
          <Col span={colSpan}>
            <Form.Item
              label={t['searchTable.columns.probability']}
              field={'probability'}
            >
              <Input placeholder={t['searchForm.placeholder']} allowClear />
            </Form.Item>
          </Col>
          <Col span={colSpan}>
            <Form.Item
              label={t['searchTable.columns.quantity']}
              field={'quantity'}
            >
              <Input placeholder={t['searchForm.placeholder']} allowClear />
            </Form.Item>
          </Col>
          <Col span={colSpan}>
            <Form.Item
              label={t['searchTable.columns.prize_desc']}
              field={'prize_desc'}
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
