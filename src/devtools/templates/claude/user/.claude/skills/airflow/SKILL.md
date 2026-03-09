---
name: airflow
description: Use when working with Apache Airflow, creating DAGs, writing Airflow tasks, configuring Airflow operators, debugging Airflow pipelines, or discussing Airflow best practices and scheduling.
tools: Read, Write, Edit, Bash, Glob, Grep
---

# Airflow

Expert guidance for working with Apache Airflow pipelines and DAGs.

## Core Concepts

- **DAG**: Directed Acyclic Graph - defines workflow
- **Task**: Unit of work within a DAG
- **Operator**: Template for a task (PythonOperator, BashOperator, etc.)
- **Sensor**: Special operator that waits for condition
- **Hook**: Interface to external systems
- **XCom**: Cross-communication between tasks

## Basic DAG Structure

```python
from datetime import datetime, timedelta
from airflow import DAG
from airflow.operators.python import PythonOperator
from airflow.operators.bash import BashOperator

# Default arguments applied to all tasks
default_args = {
    'owner': 'data-team',
    'depends_on_past': False,
    'email': ['alerts@example.com'],
    'email_on_failure': True,
    'email_on_retry': False,
    'retries': 2,
    'retry_delay': timedelta(minutes=5),
    'execution_timeout': timedelta(hours=2),
}

with DAG(
    dag_id='example_pipeline',
    default_args=default_args,
    description='Example Airflow pipeline',
    schedule='0 2 * * *',  # Run at 2 AM daily (cron format)
    start_date=datetime(2024, 1, 1),
    catchup=False,  # Don't backfill past runs
    tags=['example', 'data-pipeline'],
    max_active_runs=1,
) as dag:

    start = BashOperator(
        task_id='start',
        bash_command='echo "Pipeline started"',
    )

    def process_data(**context):
        """Process data with access to context"""
        execution_date = context['execution_date']
        print(f"Processing data for {execution_date}")
        # Your logic here
        return {'status': 'success', 'records': 1000}

    process = PythonOperator(
        task_id='process_data',
        python_callable=process_data,
        provide_context=True,
    )

    end = BashOperator(
        task_id='end',
        bash_command='echo "Pipeline completed"',
    )

    # Define dependencies
    start >> process >> end
```

## Task Dependencies

### Linear Dependencies
```python
task_a >> task_b >> task_c  # task_a then task_b then task_c
```

### Multiple Dependencies
```python
# task_d runs after both task_b and task_c complete
task_a >> [task_b, task_c] >> task_d
```

### Cross Dependencies
```python
# More complex relationships
task_a >> task_b
task_a >> task_c
[task_b, task_c] >> task_d
```

### Using set_upstream/set_downstream
```python
task_b.set_upstream(task_a)  # Same as task_a >> task_b
task_c.set_downstream(task_d)  # Same as task_c >> task_d
```

## Common Operators

### PythonOperator
```python
from airflow.operators.python import PythonOperator

def my_function(param1, param2, **context):
    """Function to execute"""
    execution_date = context['execution_date']
    print(f"Running with {param1}, {param2} on {execution_date}")
    return {'result': 'success'}

task = PythonOperator(
    task_id='python_task',
    python_callable=my_function,
    op_kwargs={'param1': 'value1', 'param2': 'value2'},
    provide_context=True,
)
```

### BashOperator
```python
from airflow.operators.bash import BashOperator

task = BashOperator(
    task_id='bash_task',
    bash_command='echo "Hello" && python script.py',
    env={'ENV_VAR': 'value'},
    cwd='/path/to/working/dir',
)
```

### EmptyOperator (formerly DummyOperator)
```python
from airflow.operators.empty import EmptyOperator

# Useful for grouping or branching logic
start = EmptyOperator(task_id='start')
end = EmptyOperator(task_id='end')
```

### BranchPythonOperator
```python
from airflow.operators.python import BranchPythonOperator

def choose_branch(**context):
    """Return task_id to execute"""
    if context['execution_date'].day % 2 == 0:
        return 'even_day_task'
    return 'odd_day_task'

branch = BranchPythonOperator(
    task_id='branch_task',
    python_callable=choose_branch,
)

even_task = EmptyOperator(task_id='even_day_task')
odd_task = EmptyOperator(task_id='odd_day_task')

branch >> [even_task, odd_task]
```

### EmailOperator
```python
from airflow.operators.email import EmailOperator

email = EmailOperator(
    task_id='send_email',
    to=['recipient@example.com'],
    subject='Airflow Alert: {{ dag.dag_id }}',
    html_content='<h3>Task completed at {{ ts }}</h3>',
)
```

## XCom (Cross-Communication)

### Push XCom
```python
def push_data(**context):
    # Automatically pushed via return value
    return {'key': 'value', 'count': 100}

    # Or explicitly push
    context['task_instance'].xcom_push(key='my_key', value='my_value')

push_task = PythonOperator(
    task_id='push_task',
    python_callable=push_data,
)
```

### Pull XCom
```python
def pull_data(**context):
    ti = context['task_instance']

    # Pull from previous task (default return value)
    data = ti.xcom_pull(task_ids='push_task')

    # Pull specific key
    value = ti.xcom_pull(task_ids='push_task', key='my_key')

    print(f"Received: {data}, {value}")

pull_task = PythonOperator(
    task_id='pull_task',
    python_callable=pull_data,
)

push_task >> pull_task
```

## Sensors

### FileSensor
```python
from airflow.sensors.filesystem import FileSensor

wait_for_file = FileSensor(
    task_id='wait_for_file',
    filepath='/path/to/file.csv',
    poke_interval=30,  # Check every 30 seconds
    timeout=3600,  # Timeout after 1 hour
    mode='poke',  # or 'reschedule' to free up worker slot
)
```

### ExternalTaskSensor
```python
from airflow.sensors.external_task import ExternalTaskSensor

wait_for_upstream = ExternalTaskSensor(
    task_id='wait_for_upstream_dag',
    external_dag_id='upstream_dag',
    external_task_id='upstream_task',
    timeout=600,
    poke_interval=60,
)
```

### Custom Sensor
```python
from airflow.sensors.base import BaseSensorOperator

class CustomSensor(BaseSensorOperator):
    def __init__(self, condition_param, **kwargs):
        super().__init__(**kwargs)
        self.condition_param = condition_param

    def poke(self, context):
        """Return True when condition is met"""
        # Check your condition
        return check_condition(self.condition_param)

sensor = CustomSensor(
    task_id='custom_sensor',
    condition_param='value',
    poke_interval=30,
)
```

## Hooks and Connections

### PostgresHook
```python
from airflow.providers.postgres.hooks.postgres import PostgresHook

def query_database(**context):
    hook = PostgresHook(postgres_conn_id='my_postgres_conn')

    # Execute query
    records = hook.get_records("SELECT * FROM users LIMIT 10")

    # Or use pandas
    df = hook.get_pandas_df("SELECT * FROM users")

    return len(records)

task = PythonOperator(
    task_id='query_db',
    python_callable=query_database,
)
```

### Custom Hook
```python
from airflow.hooks.base import BaseHook

class MyAPIHook(BaseHook):
    def __init__(self, conn_id):
        self.conn_id = conn_id
        self.connection = self.get_connection(conn_id)

    def get_data(self):
        # Use self.connection.host, .login, .password, etc.
        pass
```

## TaskFlow API (Airflow 2.0+)

Modern, Pythonic way to write DAGs:

```python
from airflow.decorators import dag, task
from datetime import datetime

@dag(
    schedule='@daily',
    start_date=datetime(2024, 1, 1),
    catchup=False,
    tags=['taskflow'],
)
def my_taskflow_dag():

    @task
    def extract():
        """Extract data"""
        return {'data': [1, 2, 3, 4, 5]}

    @task
    def transform(data: dict):
        """Transform data"""
        values = data['data']
        return {'data': [x * 2 for x in values]}

    @task
    def load(data: dict):
        """Load data"""
        print(f"Loading: {data['data']}")
        return 'success'

    # Define flow - XCom automatically handled
    data = extract()
    transformed = transform(data)
    load(transformed)

# Instantiate the DAG
dag_instance = my_taskflow_dag()
```

### TaskFlow with Multiple Outputs
```python
@task(multiple_outputs=True)
def extract_multiple():
    return {
        'users': [1, 2, 3],
        'orders': [10, 20, 30],
    }

@task
def process_users(users):
    print(f"Processing users: {users}")

@task
def process_orders(orders):
    print(f"Processing orders: {orders}")

# Usage
data = extract_multiple()
process_users(data['users'])
process_orders(data['orders'])
```

## Dynamic Task Generation

```python
from airflow.decorators import task

@dag(schedule='@daily', start_date=datetime(2024, 1, 1), catchup=False)
def dynamic_dag():

    @task
    def get_tasks():
        """Return list of items to process"""
        return ['item1', 'item2', 'item3', 'item4']

    @task
    def process_item(item: str):
        """Process individual item"""
        print(f"Processing {item}")
        return f"{item}_processed"

    # Generate tasks dynamically
    items = get_tasks()
    process_item.expand(item=items)

dag_instance = dynamic_dag()
```

## Scheduling

### Cron Expressions
```python
schedule='0 2 * * *'      # Daily at 2 AM
schedule='*/15 * * * *'   # Every 15 minutes
schedule='0 0 * * 0'      # Weekly on Sunday at midnight
schedule='0 0 1 * *'      # Monthly on 1st at midnight
```

### Presets
```python
schedule='@once'      # Run once
schedule='@hourly'    # Every hour
schedule='@daily'     # Every day at midnight
schedule='@weekly'    # Every Sunday at midnight
schedule='@monthly'   # First day of month at midnight
schedule='@yearly'    # January 1st at midnight
schedule=None         # Manual trigger only
```

### Timedelta
```python
from datetime import timedelta

schedule=timedelta(hours=2)  # Every 2 hours
```

## Error Handling and Retries

```python
from airflow.exceptions import AirflowException

def task_with_retry(**context):
    try:
        # Your logic
        result = risky_operation()
        return result
    except SpecificError as e:
        # Log and retry
        raise AirflowException(f"Operation failed: {e}")
    except Exception as e:
        # Skip retry for certain errors
        if should_skip_retry(e):
            raise AirflowSkipException(f"Skipping: {e}")
        raise

task = PythonOperator(
    task_id='retry_task',
    python_callable=task_with_retry,
    retries=3,
    retry_delay=timedelta(minutes=5),
    retry_exponential_backoff=True,
    max_retry_delay=timedelta(hours=1),
)
```

## Testing DAGs

```python
# Test that DAG loads without errors
from airflow.models import DagBag

def test_dag_loaded():
    dagbag = DagBag(dag_folder='dags/', include_examples=False)
    assert len(dagbag.import_errors) == 0, "DAG import errors"
    assert 'my_dag_id' in dagbag.dags

# Test task dependencies
def test_task_dependencies():
    dagbag = DagBag()
    dag = dagbag.get_dag('my_dag_id')

    task_a = dag.get_task('task_a')
    task_b = dag.get_task('task_b')

    assert task_b in task_a.downstream_list
```

## Common Patterns

### Parameterized DAGs
```python
@dag(
    schedule='@daily',
    start_date=datetime(2024, 1, 1),
    params={
        'env': 'dev',
        'batch_size': 1000,
    },
)
def parameterized_dag():
    @task
    def use_params(**context):
        env = context['params']['env']
        batch_size = context['params']['batch_size']
        print(f"Running in {env} with batch size {batch_size}")

    use_params()
```

### Sub-DAG Alternative (TaskGroup)
```python
from airflow.utils.task_group import TaskGroup

with DAG('main_dag', ...) as dag:

    start = EmptyOperator(task_id='start')

    with TaskGroup('processing_group') as processing:
        task1 = PythonOperator(task_id='task1', ...)
        task2 = PythonOperator(task_id='task2', ...)
        task1 >> task2

    end = EmptyOperator(task_id='end')

    start >> processing >> end
```

## Airflow CLI Commands

```bash
# DAG management
airflow dags list                          # List all DAGs
airflow dags trigger my_dag                # Trigger a DAG
airflow dags pause my_dag                  # Pause a DAG
airflow dags unpause my_dag                # Unpause a DAG
airflow dags test my_dag 2024-01-01        # Test run a DAG

# Task management
airflow tasks list my_dag                  # List tasks in DAG
airflow tasks test my_dag my_task 2024-01-01  # Test a single task

# Database
airflow db init                            # Initialize database
airflow db upgrade                         # Upgrade database schema
airflow db reset                           # Reset database (CAUTION)

# Users
airflow users create --username admin --password admin --firstname Admin --lastname User --role Admin --email admin@example.com

# Connections
airflow connections add my_conn --conn-type postgres --conn-host localhost --conn-login user --conn-password pass
airflow connections list
```

## Best Practices

1. **Idempotency**: Tasks should produce same result when run multiple times
2. **Atomicity**: Tasks should be atomic units of work
3. **Use TaskFlow API**: Simpler syntax for Python-based DAGs
4. **Avoid top-level code**: Don't execute expensive operations at DAG parse time
5. **Use Variables/Connections**: Store config in Airflow, not in code
6. **Set execution_timeout**: Prevent tasks from running indefinitely
7. **Catchup carefully**: Set `catchup=False` unless backfill is needed
8. **Tags for organization**: Use tags to categorize DAGs
9. **Document**: Add descriptions to DAGs and tasks
10. **Test DAGs**: Write unit tests for DAG structure and task logic

## When Helping Users

1. **Check Airflow version**: Syntax differs between 1.x and 2.x
2. **Prefer TaskFlow API**: Use decorators for Airflow 2.x
3. **Verify imports**: Ensure operators are imported correctly
4. **Consider dependencies**: Check task dependencies make sense
5. **Add error handling**: Include retries and failure callbacks
